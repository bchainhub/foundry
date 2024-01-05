//! Helper trait and functions to format corebc types.

use corebc_core::{types::*, utils::hex};
use serde::Deserialize;

/// Helper trait to format corebc types.
///
/// # Examples
///
/// ```
/// use foxar_macros::fmt::UIfmt;
/// let boolean: bool = true;
/// let string = boolean.pretty();
/// ```
pub trait UIfmt {
    /// Return a prettified string version of the value
    fn pretty(&self) -> String;
}

impl UIfmt for String {
    fn pretty(&self) -> String {
        self.to_string()
    }
}
impl UIfmt for bool {
    fn pretty(&self) -> String {
        self.to_string()
    }
}

impl UIfmt for U256 {
    fn pretty(&self) -> String {
        self.to_string()
    }
}

impl UIfmt for H1368 {
    fn pretty(&self) -> String {
        self.to_string()
    }
}

impl UIfmt for I256 {
    fn pretty(&self) -> String {
        self.to_string()
    }
}

impl UIfmt for Address {
    fn pretty(&self) -> String {
        self.to_string()
    }
}

impl UIfmt for H64 {
    fn pretty(&self) -> String {
        format!("{self:#x}")
    }
}

impl UIfmt for H256 {
    fn pretty(&self) -> String {
        format!("{self:#x}")
    }
}

impl UIfmt for Bytes {
    fn pretty(&self) -> String {
        format!("{self:#x}")
    }
}

impl<const N: usize> UIfmt for [u8; N] {
    fn pretty(&self) -> String {
        format!("0x{}", hex::encode(&self[..]))
    }
}

impl UIfmt for U64 {
    fn pretty(&self) -> String {
        self.to_string()
    }
}

impl UIfmt for Bloom {
    fn pretty(&self) -> String {
        format!("{self:#x}")
    }
}

impl<T: UIfmt> UIfmt for Option<T> {
    fn pretty(&self) -> String {
        if let Some(ref inner) = self {
            inner.pretty()
        } else {
            "".to_string()
        }
    }
}

impl<T: UIfmt> UIfmt for Vec<T> {
    fn pretty(&self) -> String {
        if !self.is_empty() {
            let mut s = String::with_capacity(self.len() * 64);
            s.push_str("[\n");
            for item in self {
                for line in item.pretty().lines() {
                    s.push('\t');
                    s.push_str(line);
                    s.push('\n');
                }
            }
            s.push(']');
            s
        } else {
            "[]".to_string()
        }
    }
}

impl UIfmt for TransactionReceipt {
    fn pretty(&self) -> String {
        format!(
            "
blockHash                  {}
blockNumber                {}
contractAddress            {}
cumulativeEnergyUsed       {}
energyUsed                 {}
logs                       {}
logsBloom                  {}
root                       {}
status                     {}
transactionHash            {}
transactionIndex           {}",
            self.block_hash.pretty(),
            self.block_number.pretty(),
            self.contract_address.pretty(),
            self.cumulative_energy_used.pretty(),
            self.energy_used.pretty(),
            serde_json::to_string(&self.logs).unwrap(),
            self.logs_bloom.pretty(),
            self.root.pretty(),
            self.status.pretty(),
            self.transaction_hash.pretty(),
            self.transaction_index.pretty(),
        )
    }
}

impl UIfmt for Log {
    fn pretty(&self) -> String {
        format!(
            "
address: {}
blockHash: {}
blockNumber: {}
data: {}
logIndex: {}
removed: {}
topics: {}
transactionHash: {}
transactionIndex: {}",
            self.address.pretty(),
            self.block_hash.pretty(),
            self.block_number.pretty(),
            self.data.pretty(),
            self.log_index.pretty(),
            self.removed.pretty(),
            self.topics.pretty(),
            self.transaction_hash.pretty(),
            self.transaction_index.pretty(),
        )
    }
}

impl UIfmt for Block<Transaction> {
    fn pretty(&self) -> String {
        format!(
            "
{}
transactions         {}",
            pretty_block_basics(self),
            self.transactions.pretty()
        )
    }
}

impl UIfmt for Block<TxHash> {
    fn pretty(&self) -> String {
        format!(
            "
{}
transactions:        {}",
            pretty_block_basics(self),
            self.transactions.pretty()
        )
    }
}

fn pretty_block_basics<T>(block: &Block<T>) -> String {
    format!(
        "
difficulty              {}
extraData               {}
energyLimit             {}
energyUsed              {}
hash                    {}
logsBloom               {}
miner                   {}
nonce                   {}
number                  {}
parentHash              {}
receiptsRoot            {}
sealFields              {}
sha3Uncles              {}
size                    {}
stateRoot               {}
timestamp               {}
totalDifficulty         {}",
        block.difficulty.pretty(),
        block.extra_data.pretty(),
        block.energy_limit.pretty(),
        block.energy_used.pretty(),
        block.hash.pretty(),
        block.logs_bloom.pretty(),
        block.author.pretty(),
        block.nonce.pretty(),
        block.number.pretty(),
        block.parent_hash.pretty(),
        block.receipts_root.pretty(),
        block.seal_fields.pretty(),
        block.uncles_hash.pretty(),
        block.size.pretty(),
        block.state_root.pretty(),
        block.timestamp.pretty(),
        block.total_difficulty.pretty(),
    )
}

/// Various numerical ethereum types used for pretty printing
#[derive(Debug, Clone, Deserialize)]
#[serde(untagged)]
#[allow(missing_docs)]
pub enum EthValue {
    U64(U64),
    U256(U256),
    U64Array(Vec<U64>),
    U256Array(Vec<U256>),
    Other(serde_json::Value),
}

impl From<serde_json::Value> for EthValue {
    fn from(val: serde_json::Value) -> Self {
        serde_json::from_value(val).expect("infallible")
    }
}

impl UIfmt for EthValue {
    fn pretty(&self) -> String {
        match self {
            EthValue::U64(num) => num.pretty(),
            EthValue::U256(num) => num.pretty(),
            EthValue::U64Array(arr) => arr.pretty(),
            EthValue::U256Array(arr) => arr.pretty(),
            EthValue::Other(val) => val.to_string().trim_matches('"').to_string(),
        }
    }
}

impl UIfmt for Transaction {
    fn pretty(&self) -> String {
        format!(
            "
blockHash               {}
blockNumber             {}
from                    {}
energy                  {}
energyPrice             {}
hash                    {}
input                   {}
nonce                   {}
to                      {}
transactionIndex        {}
value                   {}
signature               {}
network_id              {}",
            self.block_hash.pretty(),
            self.block_number.pretty(),
            self.from.pretty(),
            self.energy.pretty(),
            self.energy_price.pretty(),
            self.hash.pretty(),
            self.input.pretty(),
            self.nonce.pretty(),
            self.to.pretty(),
            self.transaction_index.pretty(),
            self.value.pretty(),
            self.sig.pretty(),
            self.network_id.pretty(),
        )
    }
}

/// Convert a U256 to bytes
pub fn to_bytes(uint: U256) -> Bytes {
    let mut buffer: [u8; 4 * 8] = [0; 4 * 8];
    uint.to_big_endian(&mut buffer);
    Bytes::from(buffer)
}

/// Returns the `UiFmt::pretty()` formatted attribute of the transactions
pub fn get_pretty_tx_attr(transaction: &Transaction, attr: &str) -> Option<String> {
    match attr {
        "blockHash" | "block_hash" => Some(transaction.block_hash.pretty()),
        "blockNumber" | "block_number" => Some(transaction.block_number.pretty()),
        "from" => Some(transaction.from.pretty()),
        "energy" => Some(transaction.energy.pretty()),
        "energyPrice" | "energy_price" => Some(transaction.energy_price.pretty()),
        "hash" => Some(transaction.hash.pretty()),
        "input" => Some(transaction.input.pretty()),
        "nonce" => Some(transaction.nonce.pretty()),
        "signature" => Some(transaction.sig.pretty()),
        "network_id" => Some(transaction.network_id.pretty()),
        "to" => Some(transaction.to.pretty()),
        "value" => Some(transaction.value.pretty()),
        "transactionIndex" | "transaction_index" => Some(transaction.transaction_index.pretty()),
        _ => None,
    }
}

/// Returns the `UiFmt::pretty()` formatted attribute of the given block
pub fn get_pretty_block_attr<TX>(block: &Block<TX>, attr: &str) -> Option<String> {
    match attr {
        "difficulty" => Some(block.difficulty.pretty()),
        "extraData" | "extra_data" => Some(block.extra_data.pretty()),
        "energyLimit" | "energy_limit" => Some(block.energy_limit.pretty()),
        "energyUsed" | "energy_used" => Some(block.energy_used.pretty()),
        "hash" => Some(block.hash.pretty()),
        "logsBloom" | "logs_bloom" => Some(block.logs_bloom.pretty()),
        "miner" | "author" => Some(block.author.pretty()),
        "nonce" => Some(block.nonce.pretty()),
        "number" => Some(block.number.pretty()),
        "parentHash" | "parent_hash" => Some(block.parent_hash.pretty()),
        "receiptsRoot" | "receipts_root" => Some(block.receipts_root.pretty()),
        "sealFields" | "seal_fields" => Some(block.seal_fields.pretty()),
        "sha3Uncles" | "sha_3_uncles" => Some(block.uncles_hash.pretty()),
        "size" => Some(block.size.pretty()),
        "stateRoot" | "state_root" => Some(block.state_root.pretty()),
        "timestamp" => Some(block.timestamp.pretty()),
        "totalDifficulty" | "total_difficult" => Some(block.total_difficulty.pretty()),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::str::FromStr;

    #[test]
    fn can_format_bytes32() {
        let val = hex::decode("7465737400000000000000000000000000000000000000000000000000000000")
            .unwrap();
        let mut b32 = [0u8; 32];
        b32.copy_from_slice(&val);

        assert_eq!(
            b32.pretty(),
            "0x7465737400000000000000000000000000000000000000000000000000000000"
        );
        let b: Bytes = val.into();
        assert_eq!(b.pretty(), b32.pretty());
    }

    #[test]
    fn can_pretty_print_tx() {
        let s = r#"
        {
        "blockHash": "0x02b853cf50bc1c335b70790f93d5a390a35a166bea9c895e685cc866e4961cae",
        "blockNumber": "0x1b4",
        "from": "ab36393ecaa2d3209cee16ce9b2360e327ed3c923346",
        "energy": "0x11cbbdc",
        "energyPrice": "0x22",
        "hash": "0x2642e960d3150244e298d52b5b0f024782253e6d0b2c9a01dd4858f7b4665a3f",
        "input": "0xd294f093",
        "nonce": "0xa2",
        "to": "ab36393ecaa2d3209cee16ce9b2360e327ed3c923347",
        "transactionIndex": "0x3",
        "value": "0x200",
        "index": "0x1b3",
        "network_id": "0x3",
        "signature": "0xdeadbeefdeadbeefdeadbeefdeadbeefdeadbeefdeadbeefdeadbeefdeadbeefdeadbeefdeadbeefdeadbeefdeadbeefdeadbeefdeadbeefdeadbeefdeadbeefdeadbeefdeadbeefdeadbeefdeadbeefdeadbeefdeadbeefdeadbeefdeadbeefdeadbeefdeadbeefdeadbeefdeadbeefdeadbeefdeadbeefdeadbeefdeadbeefdeadbeefdeadbeefdeadbeefdeadbeefdeadbeefdeadbeefdeadbeefdeadbeefdeadbeefdeadbeefdeadbe",
        "rawTransaction": "0xf86681a28084011cbbdc944a16a42407aa491564643e1dfc1fd50af29794ef8084d294f09338a06fca94073a0cf3381978662d46cf890602d3e9ccf6a31e4b69e8ecbd995e2beea00e804161a2b56a37ca1f6f4c4b8bce926587afa0d9b1acc5165e6556c959d583"
    }
        "#;

        let tx: Transaction = serde_json::from_str(s).unwrap();
        assert_eq!(
            tx.pretty().trim(),
            r#"
blockHash               0x02b853cf50bc1c335b70790f93d5a390a35a166bea9c895e685cc866e4961cae
blockNumber             436
from                    ab36393ecaa2d3209cee16ce9b2360e327ed3c923346
energy                  18660316
energyPrice             34
hash                    0x2642e960d3150244e298d52b5b0f024782253e6d0b2c9a01dd4858f7b4665a3f
input                   0xd294f093
nonce                   162
to                      ab36393ecaa2d3209cee16ce9b2360e327ed3c923347
transactionIndex        3
value                   512
signature               0xdead…adbe
network_id              3
"#
            .trim()
        );
    }

    #[test]
    fn print_block_w_txs() {
        let block = r#"{"number":"0x3","hash":"0xda53da08ef6a3cbde84c33e51c04f68c3853b6a3731f10baa2324968eee63972","parentHash":"0x689c70c080ca22bc0e681694fa803c1aba16a69c8b6368fed5311d279eb9de90","nonce":"0x0000000000000000","sha3Uncles":"0x1dcc4de8dec75d7aab85b567b6ccd41ad312451b948a7413f0a142fd40d49347","logsBloom":"0x00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000","transactionsRoot":"0x7270c1c4440180f2bd5215809ee3d545df042b67329499e1ab97eb759d31610d","stateRoot":"0x29f32984517a7d25607da485b23cefabfd443751422ca7e603395e1de9bc8a4b","receiptsRoot":"0x056b23fbba480696b65fe5a59b8f2148a1299103c4f57df839233af2cf4ca2d2","miner":"ab36393ecaa2d3209cee16ce9b2360e327ed3c923346","difficulty":"0x0","totalDifficulty":"0x0","extraData":"0x","size":"0x3e8","energyLimit":"0x6691b7","energyUsed":"0x5208","timestamp":"0x5ecedbb9","transactions":[{"blockHash": "0x02b853cf50bc1c335b70790f93d5a390a35a166bea9c895e685cc866e4961cae","blockNumber": "0x1b4","from": "ab36393ecaa2d3209cee16ce9b2360e327ed3c923346","energy": "0x11cbbdc","energyPrice": "0x22","hash": "0x2642e960d3150244e298d52b5b0f024782253e6d0b2c9a01dd4858f7b4665a3f","input": "0xd294f093","nonce": "0xa2","to": "ab36393ecaa2d3209cee16ce9b2360e327ed3c923347","transactionIndex": "0x3","value": "0x200","index": "0x1b3","network_id": "0x3","signature": "0xdeadbeefdeadbeefdeadbeefdeadbeefdeadbeefdeadbeefdeadbeefdeadbeefdeadbeefdeadbeefdeadbeefdeadbeefdeadbeefdeadbeefdeadbeefdeadbeefdeadbeefdeadbeefdeadbeefdeadbeefdeadbeefdeadbeefdeadbeefdeadbeefdeadbeefdeadbeefdeadbeefdeadbeefdeadbeefdeadbeefdeadbeefdeadbeefdeadbeefdeadbeefdeadbeefdeadbeefdeadbeefdeadbeefdeadbeefdeadbeefdeadbeefdeadbeefdeadbe"}],"uncles":[]}"#;
        let block: Block<Transaction> = serde_json::from_str(block).unwrap();
        let output ="\nblockHash               0x02b853cf50bc1c335b70790f93d5a390a35a166bea9c895e685cc866e4961cae
blockNumber             436
from                    ab36393ecaa2d3209cee16ce9b2360e327ed3c923346
energy                  18660316
energyPrice             34
hash                    0x2642e960d3150244e298d52b5b0f024782253e6d0b2c9a01dd4858f7b4665a3f
input                   0xd294f093
nonce                   162
to                      ab36393ecaa2d3209cee16ce9b2360e327ed3c923347
transactionIndex        3
value                   512
signature               0xdead…adbe
network_id              3".to_string();
        let generated = block.transactions[0].pretty();
        assert_eq!(generated.as_str(), output.as_str());
    }

    #[test]
    fn uifmt_option_u64() {
        let empty: Option<U64> = None;
        assert_eq!("".to_string(), empty.pretty());
        assert_eq!("100".to_string(), U64::from_dec_str("100").unwrap().pretty());
        assert_eq!("100".to_string(), Option::from(U64::from_dec_str("100").unwrap()).pretty())
    }

    #[test]
    fn uifmt_option_h64() {
        let empty: Option<H256> = None;
        assert_eq!("".to_string(), empty.pretty());
        H256::from_low_u64_be(100);
        assert_eq!(
            "0x0000000000000000000000000000000000000000000000000000000000000064",
            H256::from_low_u64_be(100).pretty()
        );
        assert_eq!(
            "0x0000000000000000000000000000000000000000000000000000000000000064",
            Some(H256::from_low_u64_be(100)).pretty()
        );
    }
    #[test]
    fn uifmt_option_bytes() {
        let empty: Option<Bytes> = None;
        assert_eq!("".to_string(), empty.pretty());
        assert_eq!(
            "0x0000000000000000000000000000000000000000000000000000000000000064".to_string(),
            Bytes::from_str("0x0000000000000000000000000000000000000000000000000000000000000064")
                .unwrap()
                .pretty()
        );
        assert_eq!(
            "0x0000000000000000000000000000000000000000000000000000000000000064".to_string(),
            Some(
                Bytes::from_str(
                    "0x0000000000000000000000000000000000000000000000000000000000000064"
                )
                .unwrap()
            )
            .pretty()
        );
    }
    #[test]
    fn test_pretty_tx_attr() {
        let block = r#"{"number":"0x3","hash":"0xda53da08ef6a3cbde84c33e51c04f68c3853b6a3731f10baa2324968eee63972","parentHash":"0x689c70c080ca22bc0e681694fa803c1aba16a69c8b6368fed5311d279eb9de90","nonce":"0x0000000000000000","sha3Uncles":"0x1dcc4de8dec75d7aab85b567b6ccd41ad312451b948a7413f0a142fd40d49347","logsBloom":"0x00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000","transactionsRoot":"0x7270c1c4440180f2bd5215809ee3d545df042b67329499e1ab97eb759d31610d","stateRoot":"0x29f32984517a7d25607da485b23cefabfd443751422ca7e603395e1de9bc8a4b","receiptsRoot":"0x056b23fbba480696b65fe5a59b8f2148a1299103c4f57df839233af2cf4ca2d2","miner":"ab36393ecaa2d3209cee16ce9b2360e327ed3c923346","difficulty":"0x0","totalDifficulty":"0x0","extraData":"0x","size":"0x3e8","energyLimit":"0x6691b7","energyUsed":"0x5208","timestamp":"0x5ecedbb9","transactions":[{"blockHash": "0x02b853cf50bc1c335b70790f93d5a390a35a166bea9c895e685cc866e4961cae","blockNumber": "0x1b4","from": "ab36393ecaa2d3209cee16ce9b2360e327ed3c923346","energy": "0x11cbbdc","energyPrice": "0x22","hash": "0x2642e960d3150244e298d52b5b0f024782253e6d0b2c9a01dd4858f7b4665a3f","input": "0xd294f093","nonce": "0xa2","to": "ab36393ecaa2d3209cee16ce9b2360e327ed3c923347","transactionIndex": "0x3","value": "0x200","index": "0x1b3","network_id": "0x3","signature": "0xdeadbeefdeadbeefdeadbeefdeadbeefdeadbeefdeadbeefdeadbeefdeadbeefdeadbeefdeadbeefdeadbeefdeadbeefdeadbeefdeadbeefdeadbeefdeadbeefdeadbeefdeadbeefdeadbeefdeadbeefdeadbeefdeadbeefdeadbeefdeadbeefdeadbeefdeadbeefdeadbeefdeadbeefdeadbeefdeadbeefdeadbeefdeadbeefdeadbeefdeadbeefdeadbeefdeadbeefdeadbeefdeadbeefdeadbeefdeadbeefdeadbeefdeadbeefdeadbe"}],"uncles":[]}"#;
        let block: Block<Transaction> = serde_json::from_str(block).unwrap();
        assert_eq!(None, get_pretty_tx_attr(&block.transactions[0], ""));
        assert_eq!(
            Some("436".to_string()),
            get_pretty_tx_attr(&block.transactions[0], "blockNumber")
        );
        assert_eq!(
            Some("ab36393ecaa2d3209cee16ce9b2360e327ed3c923346".to_string()),
            get_pretty_tx_attr(&block.transactions[0], "from")
        );
        assert_eq!(
            Some("18660316".to_string()),
            get_pretty_tx_attr(&block.transactions[0], "energy")
        );
        assert_eq!(
            Some("34".to_string()),
            get_pretty_tx_attr(&block.transactions[0], "energyPrice")
        );
        assert_eq!(
            Some("0x2642e960d3150244e298d52b5b0f024782253e6d0b2c9a01dd4858f7b4665a3f".to_string()),
            get_pretty_tx_attr(&block.transactions[0], "hash")
        );
        assert_eq!(
            Some("0xd294f093".to_string()),
            get_pretty_tx_attr(&block.transactions[0], "input")
        );
        assert_eq!(Some("162".to_string()), get_pretty_tx_attr(&block.transactions[0], "nonce"));
        assert_eq!(Some("3".to_string()), get_pretty_tx_attr(&block.transactions[0], "network_id"));
        assert_eq!(
            Some("ab36393ecaa2d3209cee16ce9b2360e327ed3c923347".into()),
            get_pretty_tx_attr(&block.transactions[0], "to")
        );
        assert_eq!(
            Some("3".to_string()),
            get_pretty_tx_attr(&block.transactions[0], "transactionIndex")
        );
        assert_eq!(Some("512".to_string()), get_pretty_tx_attr(&block.transactions[0], "value"));
        assert_eq!(
            Some("0xdead…adbe".to_string()),
            get_pretty_tx_attr(&block.transactions[0], "signature")
        );
    }
    #[test]
    fn test_pretty_block_attr() {
        let json = serde_json::json!(
        {
            "miner": "ab36393ecaa2d3209cee16ce9b2360e327ed3c923346",
            "number": "0x1b4",
            "hash": "0x0e670ec64341771606e55d6b4ca35a1a6b75ee3d5145a99d05921026d1527331",
            "parentHash": "0x9646252be9520f6e71339a8df9c55e4d7619deeb018d2a3f2d21fc165dde5eb5",
            "nonce": "0x0000000000000000",
            "sha3Uncles": "0x1dcc4de8dec75d7aab85b567b6ccd41ad312451b948a7413f0a142fd40d49347",
            "logsBloom":  "0x0e670ec64341771606e55d6b4ca35a1a6b75ee3d5145a99d05921026d15273310e670ec64341771606e55d6b4ca35a1a6b75ee3d5145a99d05921026d15273310e670ec64341771606e55d6b4ca35a1a6b75ee3d5145a99d05921026d15273310e670ec64341771606e55d6b4ca35a1a6b75ee3d5145a99d05921026d15273310e670ec64341771606e55d6b4ca35a1a6b75ee3d5145a99d05921026d15273310e670ec64341771606e55d6b4ca35a1a6b75ee3d5145a99d05921026d15273310e670ec64341771606e55d6b4ca35a1a6b75ee3d5145a99d05921026d15273310e670ec64341771606e55d6b4ca35a1a6b75ee3d5145a99d05921026d1527331",
            "transactionsRoot": "0x56e81f171bcc55a6ff8345e692c0f86e5b48e01b996cadc001622fb5e363b421",
            "receiptsRoot": "0x56e81f171bcc55a6ff8345e692c0f86e5b48e01b996cadc001622fb5e363b421",
            "stateRoot": "0xd5855eb08b3387c0af375e9cdb6acfc05eb8f519e419b874b6ff2ffda7ed1dff",
            "difficulty": "0x27f07",
            "totalDifficulty": "0x27f07",
            "extraData": "0x0000000000000000000000000000000000000000000000000000000000000000",
            "size": "0x27f07",
            "energyLimit": "0x9f759",
            "energyUsed": "0x9f759",
            "timestamp": "0x54e34e8e",
            "transactions": [],
            "uncles": []
          }
        );

        let block: Block<()> = serde_json::from_value(json).unwrap();

        assert_eq!(None, get_pretty_block_attr(&block, ""));
        assert_eq!(Some("163591".to_string()), get_pretty_block_attr(&block, "difficulty"));
        assert_eq!(
            Some("0x0000000000000000000000000000000000000000000000000000000000000000".to_string()),
            get_pretty_block_attr(&block, "extraData")
        );
        assert_eq!(Some("653145".to_string()), get_pretty_block_attr(&block, "energyLimit"));
        assert_eq!(Some("653145".to_string()), get_pretty_block_attr(&block, "energyUsed"));
        assert_eq!(
            Some("0x0e670ec64341771606e55d6b4ca35a1a6b75ee3d5145a99d05921026d1527331".to_string()),
            get_pretty_block_attr(&block, "hash")
        );
        assert_eq!(Some("0x0e670ec64341771606e55d6b4ca35a1a6b75ee3d5145a99d05921026d15273310e670ec64341771606e55d6b4ca35a1a6b75ee3d5145a99d05921026d15273310e670ec64341771606e55d6b4ca35a1a6b75ee3d5145a99d05921026d15273310e670ec64341771606e55d6b4ca35a1a6b75ee3d5145a99d05921026d15273310e670ec64341771606e55d6b4ca35a1a6b75ee3d5145a99d05921026d15273310e670ec64341771606e55d6b4ca35a1a6b75ee3d5145a99d05921026d15273310e670ec64341771606e55d6b4ca35a1a6b75ee3d5145a99d05921026d15273310e670ec64341771606e55d6b4ca35a1a6b75ee3d5145a99d05921026d1527331".to_string()),  get_pretty_block_attr(&block, "logsBloom"));
        assert_eq!(
            Some("ab36393ecaa2d3209cee16ce9b2360e327ed3c923346".to_string()),
            get_pretty_block_attr(&block, "miner")
        );
        assert_eq!(Some("0x0000000000000000".to_string()), get_pretty_block_attr(&block, "nonce"));
        assert_eq!(Some("436".to_string()), get_pretty_block_attr(&block, "number"));
        assert_eq!(
            Some("0x9646252be9520f6e71339a8df9c55e4d7619deeb018d2a3f2d21fc165dde5eb5".to_string()),
            get_pretty_block_attr(&block, "parentHash")
        );
        assert_eq!(
            Some("0x56e81f171bcc55a6ff8345e692c0f86e5b48e01b996cadc001622fb5e363b421".to_string()),
            get_pretty_block_attr(&block, "receiptsRoot")
        );
        assert_eq!(
            Some("0x1dcc4de8dec75d7aab85b567b6ccd41ad312451b948a7413f0a142fd40d49347".to_string()),
            get_pretty_block_attr(&block, "sha3Uncles")
        );
        assert_eq!(Some("163591".to_string()), get_pretty_block_attr(&block, "size"));
        assert_eq!(
            Some("0xd5855eb08b3387c0af375e9cdb6acfc05eb8f519e419b874b6ff2ffda7ed1dff".to_string()),
            get_pretty_block_attr(&block, "stateRoot")
        );
        assert_eq!(Some("1424182926".to_string()), get_pretty_block_attr(&block, "timestamp"));
        assert_eq!(Some("163591".to_string()), get_pretty_block_attr(&block, "totalDifficulty"));
    }
}
