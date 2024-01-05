use corebc::{
    prelude::{Middleware, SignerMiddleware},
    signers::Signer,
    types::{transaction::cip712::TypedData, Address, Network, TransactionRequest},
};
use shuttle::{spawn, NodeConfig};

#[tokio::test(flavor = "multi_thread")]
async fn can_sign_typed_data() {
    let (api, _handle) = spawn(NodeConfig::test()).await;

    let json = serde_json::json!(
            {
      "types": {
        "CIP712Domain": [
          {
            "name": "name",
            "type": "string"
          },
          {
            "name": "version",
            "type": "string"
          },
          {
            "name": "networkId",
            "type": "uint256"
          },
          {
            "name": "verifyingContract",
            "type": "address"
          }
        ],
        "Person": [
          {
            "name": "name",
            "type": "string"
          },
          {
            "name": "wallet",
            "type": "address"
          }
        ],
        "Mail": [
          {
            "name": "from",
            "type": "Person"
          },
          {
            "name": "to",
            "type": "Person"
          },
          {
            "name": "contents",
            "type": "string"
          }
        ]
      },
      "primaryType": "Mail",
      "domain": {
        "name": "Ether Mail",
        "version": "1",
        "networkId": 1,
        "verifyingContract": "0xccccCcCCccccCCCCcCCCCCCcCcCccCcCCCcCcccccccC"
      },
      "message": {
        "from": {
          "name": "Cow",
          "wallet": "0xcb66CD2a3d9F938E13CD947Ec05AbC7FE734Df8DD826"
        },
        "to": {
          "name": "Bob",
          "wallet": "0xbbbbbBbBBBBbbBBBbbbBbbBbbbbBBbBbbbbBbBbbBBbB"
        },
        "contents": "Hello, Bob!"
      }
    });

    let typed_data: TypedData = serde_json::from_value(json).unwrap();

    // `curl -X POST -H "Content-Type: application/json" -d '{"jsonrpc":"2.0","method": "eth_signTypedData_v4", "params": ["cb77531c365fa0f1d46d65440e95c3ba6a2d21a62d85", {"types":{"CIP712Domain":[{"name":"name","type":"string"},{"name":"version","type":"string"},{"name":"networkId","type":"uint256"},{"name":"verifyingContract","type":"address"}],"Person":[{"name":"name","type":"string"},{"name":"wallet","type":"address"}],"Mail":[{"name":"from","type":"Person"},{"name":"to","type":"Person"},{"name":"contents","type":"string"}]},"primaryType":"Mail","domain":{"name":"Ether Mail","version":"1","networkId":1,"verifyingContract":"0xCcCCccccCCCCcCCCCCCcCcCccCcCCCcCcccccccC"},"message":{"from":{"name":"Cow","wallet":"0xCD2a3d9F938E13CD947Ec05AbC7FE734Df8DD826"},"to":{"name":"Bob","wallet":"0xbBbBBBBbbBBBbbbBbbBbbbbBBbBbbbbBbBbbBBbB"},"contents":"Hello, Bob!"}}],"id":67}' http://localhost:8545`

    let signature = api
        .sign_typed_data_v4(
            "0xcb58e5dd06163a480c22d540ec763325a0b5860fb56c".parse().unwrap(),
            &typed_data,
        )
        .await
        .unwrap();
    assert_eq!(
      signature,
      "0x999f54339b16d01aa7e782bb991959b2a08d1aebcd07b378fdb5244d4c525f96722fc518dd40bbbe65425360901689aee1cbd5ba16d266cc8055431991092c7487b4a5e6af70af7eade3e697c2c2ed587eee15a9a15e6a20768f9d758e530a7929fbaf3ad9ee2dd79e07e392d121b05c2100cce9b032abc93b3607188cb68fb3630ef23b9dabf1a8860baa1420532d2285bca721427d5075a4678bdc61d6e380b4998ac48172805ad7c580".to_string()
    );
}

// <https://github.com/foxar-rs/foxar/issues/2458>
#[tokio::test(flavor = "multi_thread")]
async fn can_sign_typed_data_os() {
    let (api, _handle) = spawn(NodeConfig::test()).await;

    let json = serde_json::json!(
    {
      "types": {
        "CIP712Domain": [
          {
            "name": "name",
            "type": "string"
          },
          {
            "name": "version",
            "type": "string"
          },
          {
            "name": "networkId",
            "type": "uint256"
          },
          {
            "name": "verifyingContract",
            "type": "address"
          }
        ],
        "OrderComponents": [
          {
            "name": "offerer",
            "type": "address"
          },
          {
            "name": "zone",
            "type": "address"
          },
          {
            "name": "offer",
            "type": "OfferItem[]"
          },
          {
            "name": "consideration",
            "type": "ConsiderationItem[]"
          },
          {
            "name": "orderType",
            "type": "uint8"
          },
          {
            "name": "startTime",
            "type": "uint256"
          },
          {
            "name": "endTime",
            "type": "uint256"
          },
          {
            "name": "zoneHash",
            "type": "bytes32"
          },
          {
            "name": "salt",
            "type": "uint256"
          },
          {
            "name": "conduitKey",
            "type": "bytes32"
          },
          {
            "name": "counter",
            "type": "uint256"
          }
        ],
        "OfferItem": [
          {
            "name": "itemType",
            "type": "uint8"
          },
          {
            "name": "token",
            "type": "address"
          },
          {
            "name": "identifierOrCriteria",
            "type": "uint256"
          },
          {
            "name": "startAmount",
            "type": "uint256"
          },
          {
            "name": "endAmount",
            "type": "uint256"
          }
        ],
        "ConsiderationItem": [
          {
            "name": "itemType",
            "type": "uint8"
          },
          {
            "name": "token",
            "type": "address"
          },
          {
            "name": "identifierOrCriteria",
            "type": "uint256"
          },
          {
            "name": "startAmount",
            "type": "uint256"
          },
          {
            "name": "endAmount",
            "type": "uint256"
          },
          {
            "name": "recipient",
            "type": "address"
          }
        ]
      },
      "primaryType": "OrderComponents",
      "domain": {
        "name": "Seaport",
        "version": "1.1",
        "networkId": "1",
        "verifyingContract": "0x000000000000006c3852cbEf3e08E8dF289169EdE581"
      },
      "message": {
        "offerer": "0xcb58e5dd06163a480c22d540ec763325a0b5860fb56c",
        "offer": [
          {
            "itemType": "3",
            "token": "0x0000A604060890923Ff400e8c6f5290461A83AEDACec",
            "identifierOrCriteria": "110194434039389003190498847789203126033799499726478230611233094448886344768909",
            "startAmount": "1",
            "endAmount": "1"
          }
        ],
        "consideration": [
          {
            "itemType": "0",
            "token": "0x00000000000000000000000000000000000000000000",
            "identifierOrCriteria": "0",
            "startAmount": "487500000000000000",
            "endAmount": "487500000000000000",
            "recipient": "0xcb58e5dd06163a480c22d540ec763325a0b5860fb56c"
          },
          {
            "itemType": "0",
            "token": "0x00000000000000000000000000000000000000000000",
            "identifierOrCriteria": "0",
            "startAmount": "12500000000000000",
            "endAmount": "12500000000000000",
            "recipient": "0x00008De9C5A032463C561423387a9648c5C7BCC5BC90"
          }
        ],
        "startTime": "1658645591",
        "endTime": "1659250386",
        "orderType": "3",
        "zone": "0x0000004C00500000aD104D7DBd00e3ae0A5C00560C00",
        "zoneHash": "0x0000000000000000000000000000000000000000000000000000000000000000",
        "salt": "16178208897136618",
        "conduitKey": "0x0000007b02230091a7ed01230072f7006a004d60a8d4e71d599b8104250f0000",
        "totalOriginalConsiderationItems": "2",
        "counter": "0"
      }
    }
        );

    let typed_data: TypedData = serde_json::from_value(json).unwrap();

    // `curl -X POST http://localhost:8545 -d '{"jsonrpc": "2.0", "method": "eth_signTypedData_v4", "params": ["cb77531c365fa0f1d46d65440e95c3ba6a2d21a62d85", {"types":{"CIP712Domain":[{"name":"name","type":"string"},{"name":"version","type":"string"},{"name":"networkId","type":"uint256"},{"name":"verifyingContract","type":"address"}],"OrderComponents":[{"name":"offerer","type":"address"},{"name":"zone","type":"address"},{"name":"offer","type":"OfferItem[]"},{"name":"consideration","type":"ConsiderationItem[]"},{"name":"orderType","type":"uint8"},{"name":"startTime","type":"uint256"},{"name":"endTime","type":"uint256"},{"name":"zoneHash","type":"bytes32"},{"name":"salt","type":"uint256"},{"name":"conduitKey","type":"bytes32"},{"name":"counter","type":"uint256"}],"OfferItem":[{"name":"itemType","type":"uint8"},{"name":"token","type":"address"},{"name":"identifierOrCriteria","type":"uint256"},{"name":"startAmount","type":"uint256"},{"name":"endAmount","type":"uint256"}],"ConsiderationItem":[{"name":"itemType","type":"uint8"},{"name":"token","type":"address"},{"name":"identifierOrCriteria","type":"uint256"},{"name":"startAmount","type":"uint256"},{"name":"endAmount","type":"uint256"},{"name":"recipient","type":"address"}]},"primaryType":"OrderComponents","domain":{"name":"Seaport","version":"1.1","networkId":"1","verifyingContract":"0x00000000006c3852cbEf3e08E8dF289169EdE581"},"message":{"offerer":"cb77531c365fa0f1d46d65440e95c3ba6a2d21a62d85","offer":[{"itemType":"3","token":"0xA604060890923Ff400e8c6f5290461A83AEDACec","identifierOrCriteria":"110194434039389003190498847789203126033799499726478230611233094448886344768909","startAmount":"1","endAmount":"1"}],"consideration":[{"itemType":"0","token":"0x0000000000000000000000000000000000000000","identifierOrCriteria":"0","startAmount":"487500000000000000","endAmount":"487500000000000000","recipient":"cb77531c365fa0f1d46d65440e95c3ba6a2d21a62d85"},{"itemType":"0","token":"0x0000000000000000000000000000000000000000","identifierOrCriteria":"0","startAmount":"12500000000000000","endAmount":"12500000000000000","recipient":"0x8De9C5A032463C561423387a9648c5C7BCC5BC90"}],"startTime":"1658645591","endTime":"1659250386","orderType":"3","zone":"0x004C00500000aD104D7DBd00e3ae0A5C00560C00","zoneHash":"0x0000000000000000000000000000000000000000000000000000000000000000","salt":"16178208897136618","conduitKey":"0x0000007b02230091a7ed01230072f7006a004d60a8d4e71d599b8104250f0000","totalOriginalConsiderationItems":"2","counter":"0"}}], "id": "1"}' -H "Content-Type: application/json"`

    let signature = api
        .sign_typed_data_v4(
            "0xcb58e5dd06163a480c22d540ec763325a0b5860fb56c".parse().unwrap(),
            &typed_data,
        )
        .await
        .unwrap();

    assert_eq!(
      signature,
      "0xa679512e3783293b11d2f39e11b04c84a7b0807ffab74b8be9273eaf5701fa1de16447bc12314d317222663a9367267bcaeef92227d8c56b0098d621c351a9f1bcfc79f08557976406fcd6bd74480c08e6289aafdd96fdd7845eee179dc0501650b1e7df17c2891f19315be0bf30c3932700cce9b032abc93b3607188cb68fb3630ef23b9dabf1a8860baa1420532d2285bca721427d5075a4678bdc61d6e380b4998ac48172805ad7c580".to_string()
    );
}

// #[tokio::test(flavor = "multi_thread")]
// async fn rejects_different_chain_id() {
//     let (_api, handle) = spawn(NodeConfig::test()).await;
//     let provider = handle.http_provider();

//     let wallet = handle.dev_wallets().next().unwrap();
//     let client = SignerMiddleware::new(provider, wallet.with_network_id(Network::Devin));

//     let tx = TransactionRequest::new().to(Address::random()).value(100u64);

//     let res = client.send_transaction(tx, None).await;
//     let err = res.unwrap_err();
//     assert!(err.to_string().contains("signed for another chain"));
// }

#[tokio::test(flavor = "multi_thread")]
async fn rejects_invalid_chain_id() {
    let (_api, handle) = spawn(NodeConfig::test()).await;
    let wallet = handle.dev_wallets().next().unwrap().with_network_id(99u64);
    let provider = handle.http_provider();
    let client = SignerMiddleware::new(provider, wallet);
    let tx = TransactionRequest::new().to(Address::random()).value(100u64);
    let res = client.send_transaction(tx, None).await;
    let _err = res.unwrap_err();
}

// <https://github.com/foxar-rs/foxar/issues/3409>
#[tokio::test(flavor = "multi_thread")]
async fn can_sign_typed_seaport_data() {
    let (api, _handle) = spawn(NodeConfig::test()).await;

    let json = serde_json::json!(
       {
      "types": {
        "CIP712Domain": [
          {
            "name": "name",
            "type": "string"
          },
          {
            "name": "version",
            "type": "string"
          },
          {
            "name": "networkId",
            "type": "uint256"
          },
          {
            "name": "verifyingContract",
            "type": "address"
          }
        ],
        "OrderComponents": [
          {
            "name": "offerer",
            "type": "address"
          },
          {
            "name": "zone",
            "type": "address"
          },
          {
            "name": "offer",
            "type": "OfferItem[]"
          },
          {
            "name": "consideration",
            "type": "ConsiderationItem[]"
          },
          {
            "name": "orderType",
            "type": "uint8"
          },
          {
            "name": "startTime",
            "type": "uint256"
          },
          {
            "name": "endTime",
            "type": "uint256"
          },
          {
            "name": "zoneHash",
            "type": "bytes32"
          },
          {
            "name": "salt",
            "type": "uint256"
          },
          {
            "name": "conduitKey",
            "type": "bytes32"
          },
          {
            "name": "counter",
            "type": "uint256"
          }
        ],
        "OfferItem": [
          {
            "name": "itemType",
            "type": "uint8"
          },
          {
            "name": "token",
            "type": "address"
          },
          {
            "name": "identifierOrCriteria",
            "type": "uint256"
          },
          {
            "name": "startAmount",
            "type": "uint256"
          },
          {
            "name": "endAmount",
            "type": "uint256"
          }
        ],
        "ConsiderationItem": [
          {
            "name": "itemType",
            "type": "uint8"
          },
          {
            "name": "token",
            "type": "address"
          },
          {
            "name": "identifierOrCriteria",
            "type": "uint256"
          },
          {
            "name": "startAmount",
            "type": "uint256"
          },
          {
            "name": "endAmount",
            "type": "uint256"
          },
          {
            "name": "recipient",
            "type": "address"
          }
        ]
      },
      "primaryType": "OrderComponents",
      "domain": {
        "name": "Seaport",
        "version": "1.1",
        "networkId": "137",
        "verifyingContract": "0x000000000000006c3852cbEf3e08E8dF289169EdE581"
      },
      "message": {
        "offerer": "cb77531c365fa0f1d46d65440e95c3ba6a2d21a62d85",
        "offer": [
          {
            "itemType": "3",
            "token": "0x0000A604060890923Ff400e8c6f5290461A83AEDACec",
            "identifierOrCriteria": "110194434039389003190498847789203126033799499726478230611233094448886344768909",
            "startAmount": "1",
            "endAmount": "1"
          }
        ],
        "consideration": [
          {
            "itemType": "0",
            "token": "0x00000000000000000000000000000000000000000000",
            "identifierOrCriteria": "0",
            "startAmount": "487500000000000000",
            "endAmount": "487500000000000000",
            "recipient": "cb77531c365fa0f1d46d65440e95c3ba6a2d21a62d85"
          },
          {
            "itemType": "0",
            "token": "0x00000000000000000000000000000000000000000000",
            "identifierOrCriteria": "0",
            "startAmount": "12500000000000000",
            "endAmount": "12500000000000000",
            "recipient": "0x00008De9C5A032463C561423387a9648c5C7BCC5BC90"
          }
        ],
        "startTime": "1658645591",
        "endTime": "1659250386",
        "orderType": "3",
        "zone": "0x0000004C00500000aD104D7DBd00e3ae0A5C00560C00",
        "zoneHash": "0x0000000000000000000000000000000000000000000000000000000000000000",
        "salt": "16178208897136618",
        "conduitKey": "0x0000007b02230091a7ed01230072f7006a004d60a8d4e71d599b8104250f0000",
        "totalOriginalConsiderationItems": "2",
        "counter": "0"
      }
    }
            );

    let typed_data: TypedData = serde_json::from_value(json).unwrap();

    // `curl -X POST http://localhost:8545 -d '{"jsonrpc": "2.0", "method": "eth_signTypedData_v4", "params": ["cb77531c365fa0f1d46d65440e95c3ba6a2d21a62d85", "{\"types\":{\"CIP712Domain\":[{\"name\":\"name\",\"type\":\"string\"},{\"name\":\"version\",\"type\":\"string\"},{\"name\":\"networkId\",\"type\":\"uint256\"},{\"name\":\"verifyingContract\",\"type\":\"address\"}],\"OrderComponents\":[{\"name\":\"offerer\",\"type\":\"address\"},{\"name\":\"zone\",\"type\":\"address\"},{\"name\":\"offer\",\"type\":\"OfferItem[]\"},{\"name\":\"consideration\",\"type\":\"ConsiderationItem[]\"},{\"name\":\"orderType\",\"type\":\"uint8\"},{\"name\":\"startTime\",\"type\":\"uint256\"},{\"name\":\"endTime\",\"type\":\"uint256\"},{\"name\":\"zoneHash\",\"type\":\"bytes32\"},{\"name\":\"salt\",\"type\":\"uint256\"},{\"name\":\"conduitKey\",\"type\":\"bytes32\"},{\"name\":\"counter\",\"type\":\"uint256\"}],\"OfferItem\":[{\"name\":\"itemType\",\"type\":\"uint8\"},{\"name\":\"token\",\"type\":\"address\"},{\"name\":\"identifierOrCriteria\",\"type\":\"uint256\"},{\"name\":\"startAmount\",\"type\":\"uint256\"},{\"name\":\"endAmount\",\"type\":\"uint256\"}],\"ConsiderationItem\":[{\"name\":\"itemType\",\"type\":\"uint8\"},{\"name\":\"token\",\"type\":\"address\"},{\"name\":\"identifierOrCriteria\",\"type\":\"uint256\"},{\"name\":\"startAmount\",\"type\":\"uint256\"},{\"name\":\"endAmount\",\"type\":\"uint256\"},{\"name\":\"recipient\",\"type\":\"address\"}]},\"primaryType\":\"OrderComponents\",\"domain\":{\"name\":\"Seaport\",\"version\":\"1.1\",\"networkId\":\"137\",\"verifyingContract\":\"0x00000000006c3852cbEf3e08E8dF289169EdE581\"},\"message\":{\"offerer\":\"cb77531c365fa0f1d46d65440e95c3ba6a2d21a62d85\",\"offer\":[{\"itemType\":\"3\",\"token\":\"0xA604060890923Ff400e8c6f5290461A83AEDACec\",\"identifierOrCriteria\":\"110194434039389003190498847789203126033799499726478230611233094448886344768909\",\"startAmount\":\"1\",\"endAmount\":\"1\"}],\"consideration\":[{\"itemType\":\"0\",\"token\":\"0x0000000000000000000000000000000000000000\",\"identifierOrCriteria\":\"0\",\"startAmount\":\"487500000000000000\",\"endAmount\":\"487500000000000000\",\"recipient\":\"cb77531c365fa0f1d46d65440e95c3ba6a2d21a62d85\"},{\"itemType\":\"0\",\"token\":\"0x0000000000000000000000000000000000000000\",\"identifierOrCriteria\":\"0\",\"startAmount\":\"12500000000000000\",\"endAmount\":\"12500000000000000\",\"recipient\":\"0x8De9C5A032463C561423387a9648c5C7BCC5BC90\"}],\"startTime\":\"1658645591\",\"endTime\":\"1659250386\",\"orderType\":\"3\",\"zone\":\"0x004C00500000aD104D7DBd00e3ae0A5C00560C00\",\"zoneHash\":\"0x0000000000000000000000000000000000000000000000000000000000000000\",\"salt\":\"16178208897136618\",\"conduitKey\":\"0x0000007b02230091a7ed01230072f7006a004d60a8d4e71d599b8104250f0000\",\"totalOriginalConsiderationItems\":\"2\",\"counter\":\"0\"}}"], "id": "1"}' -H "Content-Type: application/json"`

    let signature = api
        .sign_typed_data_v4(
            "0xcb58e5dd06163a480c22d540ec763325a0b5860fb56c".parse().unwrap(),
            &typed_data,
        )
        .await
        .unwrap();

    assert_eq!(
    signature,
    "0x1e946940e6e1a9a2180974b3706b917c790233c44f6d40ce98f43da6d311f089aebb565e539f3f696e2a2f9eaf112416a3b7ffff16b4b1a480e476080d0045525d3923b4730c1d83f4e149447463b79caf650f8684751e4e3e77b796be4a3ca99691cf04e0081f3f5cf9f10aeae7c0c40800cce9b032abc93b3607188cb68fb3630ef23b9dabf1a8860baa1420532d2285bca721427d5075a4678bdc61d6e380b4998ac48172805ad7c580".to_string()
  );
}
