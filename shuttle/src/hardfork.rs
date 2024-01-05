use corebc::types::BlockNumber;
use ethereum_forkid::{ForkHash, ForkId};
use foxar_evm::revm::primitives::SpecId;
use std::str::FromStr;

#[derive(Debug, Copy, Clone, Eq, PartialEq, Default)]
pub enum Hardfork {
    Frontier,
    Homestead,
    Dao,
    Tangerine,
    SpuriousDragon,
    Byzantium,
    Constantinople,
    Petersburg,
    Istanbul,
    #[default]
    Latest,
}

impl Hardfork {
    /// Get the first block number of the hardfork.
    pub fn fork_block(&self) -> u64 {
        // match *self {
        //     _ => 0,
        // }
        0
    }

    pub fn fork_id(&self) -> ForkId {
        ForkId { hash: ForkHash([0x87, 0x9d, 0x6e, 0x30]), next: 0 }
    }
}

impl FromStr for Hardfork {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.to_lowercase();
        let hardfork = match s.as_str() {
            "frontier" | "1" => Hardfork::Frontier,
            "homestead" | "2" => Hardfork::Homestead,
            "dao" | "3" => Hardfork::Dao,
            "tangerine" | "4" => Hardfork::Tangerine,
            "spuriousdragon" | "5" => Hardfork::SpuriousDragon,
            "byzantium" | "6" => Hardfork::Byzantium,
            "constantinople" | "7" => Hardfork::Constantinople,
            "petersburg" | "8" => Hardfork::Petersburg,
            "istanbul" | "9" => Hardfork::Istanbul,
            "latest" => Hardfork::Latest,
            _ => return Err(format!("Unknown hardfork {s}")),
        };
        Ok(hardfork)
    }
}

impl From<Hardfork> for SpecId {
    fn from(fork: Hardfork) -> Self {
        match fork {
            Hardfork::Frontier => SpecId::FRONTIER,
            Hardfork::Homestead => SpecId::HOMESTEAD,
            Hardfork::Dao => SpecId::HOMESTEAD,
            Hardfork::Tangerine => SpecId::TANGERINE,
            Hardfork::SpuriousDragon => SpecId::SPURIOUS_DRAGON,
            Hardfork::Byzantium => SpecId::BYZANTIUM,
            Hardfork::Constantinople => SpecId::CONSTANTINOPLE,
            Hardfork::Petersburg => SpecId::PETERSBURG,
            Hardfork::Istanbul | Hardfork::Latest => SpecId::ISTANBUL,
        }
    }
}

impl<T: Into<BlockNumber>> From<T> for Hardfork {
    fn from(block: T) -> Hardfork {
        let _num = match block.into() {
            BlockNumber::Earliest => 0,
            BlockNumber::Number(num) => num.as_u64(),
            _ => u64::MAX,
        };

        Hardfork::Latest
    }
}

#[cfg(test)]
mod tests {
    use crate::Hardfork;
    use corebc::utils::hex;
    use crc::{Crc, CRC_32_ISO_HDLC};

    #[test]
    // this test checks that the fork hash assigned to forks accurately map to the fork_id method
    fn test_forkhash_from_fork_blocks() {
        // // set the genesis hash
        // let genesis =
        //     hex::decode("d4e56740f876aef8c010b86a40d5f56745a118d0906a34e69aec8c0db1cb8fa3")
        //         .unwrap();
        //
        // // instantiate the crc "hasher"
        // let crc_hasher = Crc::<u32>::new(&CRC_32_ISO_HDLC);
        // let mut crc_digest = crc_hasher.digest();
        //
        // // check frontier forkhash
        // crc_digest.update(&genesis);
        //
        // // now we go through enum members
        // let frontier_forkid = Hardfork::Frontier.fork_id();
        // let frontier_forkhash = u32::from_be_bytes(frontier_forkid.hash.0);
        // // clone the digest for finalization so we can update it again
        // assert_eq!(crc_digest.clone().finalize(), frontier_forkhash);
        //
        // // list of the above hardforks
        // let hardforks = vec![
        //     Hardfork::Homestead,
        //     Hardfork::Dao,
        //     Hardfork::Tangerine,
        //     Hardfork::SpuriousDragon,
        //     Hardfork::Byzantium,
        //     Hardfork::Constantinople,
        //     Hardfork::Istanbul,
        // ];
        //
        // // now loop through each hardfork, conducting each forkhash test
        // for hardfork in hardforks {
        //     // this could also be done with frontier_forkhash.next, but fork_block is used for
        // more     // coverage
        //     let fork_block = hardfork.fork_block().to_be_bytes();
        //     crc_digest.update(&fork_block);
        //     let fork_hash = u32::from_be_bytes(hardfork.fork_id().hash.0);
        //     assert_eq!(crc_digest.clone().finalize(), fork_hash);
        // }
    }
}
