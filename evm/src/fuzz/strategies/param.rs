use super::state::EvmFuzzState;
use corebc::{
    abi::{ParamType, Token, Tokenizable},
    types::{Bytes, Network, H160, I256, U256},
    utils::to_ican,
};
use proptest::prelude::*;

/// The max length of arrays we fuzz for is 256.
pub const MAX_ARRAY_LEN: usize = 256;

/// Given a parameter type, returns a strategy for generating values for that type.
///
/// Works with ABI Encoder v2 tuples.
pub fn fuzz_param(param: &ParamType, network: Network) -> impl Strategy<Value = Token> {
    println!("{:?}", param);
    match param {
        ParamType::Address => {
            // The key to making this work is the `boxed()` call which type erases everything
            // https://altsysrq.github.io/proptest-book/proptest/tutorial/transforming-strategies.html
            any::<[u8; 20]>()
                .prop_map(move |x| to_ican(&H160::from_slice(&x), &network).into_token())
                .boxed()
        }
        ParamType::Bytes => any::<Vec<u8>>().prop_map(|x| Bytes::from(x).into_token()).boxed(),
        ParamType::Int(n) => {
            super::IntStrategy::new(*n, vec![]).prop_map(|x| x.into_token()).boxed()
        }
        ParamType::Uint(n) => {
            super::UintStrategy::new(*n, vec![]).prop_map(|x| x.into_token()).boxed()
        }
        ParamType::Bool => any::<bool>().prop_map(|x| x.into_token()).boxed(),
        ParamType::String => any::<Vec<u8>>()
            .prop_map(|x| Token::String(unsafe { std::str::from_utf8_unchecked(&x).to_string() }))
            .boxed(),
        ParamType::Array(param) => {
            proptest::collection::vec(fuzz_param(param, network), 0..MAX_ARRAY_LEN)
                .prop_map(Token::Array)
                .boxed()
        }
        ParamType::FixedBytes(size) => {
            let res = (0..*size as u64)
            .map(|_| any::<u8>())
            .collect::<Vec<_>>()
            .prop_map(Token::FixedBytes);
            println!("{:?}", res.clone());
            
            res.boxed()
        },
        ParamType::FixedArray(param, size) => std::iter::repeat_with(|| {
            fuzz_param(param, network).prop_map(|param| param.into_token())
        })
        .take(*size)
        .collect::<Vec<_>>()
        .prop_map(Token::FixedArray)
        .boxed(),
        ParamType::Tuple(params) => params
            .iter()
            .map(|x| fuzz_param(x, network))
            .collect::<Vec<_>>()
            .prop_map(Token::Tuple)
            .boxed(),
    }
}

/// Given a parameter type, returns a strategy for generating values for that type, given some EVM
/// fuzz state.
///
/// Works with ABI Encoder v2 tuples.
pub fn fuzz_param_from_state(
    param: &ParamType,
    arc_state: EvmFuzzState,
    network: Network,
) -> BoxedStrategy<Token> {
    // These are to comply with lifetime requirements
    let state_len = arc_state.read().values().len();

    // Select a value from the state
    let st = arc_state.clone();
    let value = any::<prop::sample::Index>()
        .prop_map(move |index| index.index(state_len))
        .prop_map(move |index| *st.read().values().iter().nth(index).unwrap());

    // Convert the value based on the parameter type
    match param {
        ParamType::Address => value
            .prop_map(move |value| to_ican(&H160::from_slice(&value[12..]), &network).into_token())
            .boxed(),
        ParamType::Bytes => value.prop_map(move |value| Bytes::from(value).into_token()).boxed(),
        ParamType::Int(n) => match n / 8 {
            32 => {
                value.prop_map(move |value| I256::from_raw(U256::from(value)).into_token()).boxed()
            }
            y @ 1..=31 => value
                .prop_map(move |value| {
                    // Generate a uintN in the correct range, then shift it to the range of intN
                    // by subtracting 2^(N-1)
                    let uint = U256::from(value) % U256::from(2usize).pow(U256::from(y * 8));
                    let max_int_plus1 = U256::from(2usize).pow(U256::from(y * 8 - 1));
                    let num = I256::from_raw(uint.overflowing_sub(max_int_plus1).0);
                    num.into_token()
                })
                .boxed(),
            _ => panic!("unsupported solidity type int{n}"),
        },
        ParamType::Uint(n) => match n / 8 {
            32 => value.prop_map(move |value| U256::from(value).into_token()).boxed(),
            y @ 1..=31 => value
                .prop_map(move |value| {
                    (U256::from(value) % (U256::from(2usize).pow(U256::from(y * 8)))).into_token()
                })
                .boxed(),
            _ => panic!("unsupported solidity type uint{n}"),
        },
        ParamType::Bool => value.prop_map(move |value| Token::Bool(value[31] == 1)).boxed(),
        ParamType::String => value
            .prop_map(move |value| {
                Token::String(
                    String::from_utf8_lossy(&value[..]).trim().trim_end_matches('\0').to_string(),
                )
            })
            .boxed(),
        ParamType::Array(param) => proptest::collection::vec(
            fuzz_param_from_state(param, arc_state, network),
            0..MAX_ARRAY_LEN,
        )
        .prop_map(Token::Array)
        .boxed(),
        ParamType::FixedBytes(size) => {
            let size = *size;
            value.prop_map(move |value| Token::FixedBytes(value[32 - size..].to_vec())).boxed()
        }
        ParamType::FixedArray(param, size) => {
            let fixed_size = *size;
            proptest::collection::vec(fuzz_param_from_state(param, arc_state, network), fixed_size)
                .prop_map(Token::FixedArray)
                .boxed()
        }
        ParamType::Tuple(params) => params
            .iter()
            .map(|p| fuzz_param_from_state(p, arc_state.clone(), network))
            .collect::<Vec<_>>()
            .prop_map(Token::Tuple)
            .boxed(),
    }
}

#[cfg(test)]
mod tests {
    use crate::fuzz::strategies::{build_initial_state, fuzz_calldata, fuzz_calldata_from_state};
    use corebc::abi::HumanReadableParser;
    use foundry_config::FuzzDictionaryConfig;
    use revm::db::{CacheDB, EmptyDB};

    #[test]
    fn can_fuzz_array() {
        let f = "function testArray(uint64[2] calldata values)";
        let func = HumanReadableParser::parse_function(f).unwrap();

        let db = CacheDB::new(EmptyDB());
        let state = build_initial_state(&db, &FuzzDictionaryConfig::default());

        let strat = proptest::strategy::Union::new_weighted(vec![
            (60, fuzz_calldata(func.clone(), &corebc::types::Network::Mainnet)),
            (40, fuzz_calldata_from_state(func, state, &corebc::types::Network::Mainnet)),
        ]);

        let cfg = proptest::test_runner::Config { failure_persistence: None, ..Default::default() };
        let mut runner = proptest::test_runner::TestRunner::new(cfg);

        let _ = runner.run(&strat, |_| Ok(()));
    }
}
