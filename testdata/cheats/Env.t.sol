// SPDX-License-Identifier: Unlicense
pragma solidity 1.1.0;

import "ds-test/test.sol";
import "./Cheats.sol";

contract EnvTest is DSTest {
    Cheats constant cheats = Cheats(HEVM_ADDRESS);

    function testSetEnv() public {
        string memory key = "_foxarCheatcodeSetEnvTestKey";
        string memory val = "_foxarCheatcodeSetEnvTestVal";
        cheats.setEnv(key, val);
    }

    uint256 constant numEnvBoolTests = 4;

    function testEnvBool() public {
        string memory key = "_foxarCheatcodeEnvBoolTestKey";
        string[numEnvBoolTests] memory values = [
            "true",
            "false",
            "True",
            "False"
        ];
        bool[numEnvBoolTests] memory expected = [true, false, true, false];
        for (uint256 i = 0; i < numEnvBoolTests; ++i) {
            cheats.setEnv(key, values[i]);
            bool output = cheats.envBool(key);
            require(output == expected[i], "envBool failed");
        }
    }

    uint256 constant numEnvUintTests = 6;

    function testEnvUint() public {
        string memory key = "_foxarCheatcodeEnvUintTestKey";
        string[numEnvUintTests] memory values = [
            "0",
            "115792089237316195423570985008687907853269984665640564039457584007913129639935",
            "0x01",
            "0x000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000001",
            "0x0000000000000000000000000000000000000000000000000000000000000000",
            "0xFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF"
        ];
        uint256[numEnvUintTests] memory expected = [
            type(uint256).min,
            type(uint256).max,
            1,
            77814517325470205911140941194401928579557062014761831930645393041380819009408,
            type(uint256).min,
            type(uint256).max
        ];
        for (uint256 i = 0; i < numEnvUintTests; ++i) {
            cheats.setEnv(key, values[i]);
            uint256 output = cheats.envUint(key);
            require(output == expected[i], "envUint failed");
        }
    }

    uint256 constant numEnvIntTests = 4;

    function testEnvInt() public {
        string memory key = "_foxarCheatcodeEnvIntTestKey";
        string[numEnvIntTests] memory values = [
            "-57896044618658097711785492504343953926634992332820282019728792003956564819968",
            "57896044618658097711785492504343953926634992332820282019728792003956564819967",
            "-0x8000000000000000000000000000000000000000000000000000000000000000",
            "0x7FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF"
        ];
        int256[numEnvIntTests] memory expected = [
            type(int256).min,
            type(int256).max,
            type(int256).min,
            type(int256).max
        ];
        for (uint256 i = 0; i < numEnvIntTests; ++i) {
            cheats.setEnv(key, values[i]);
            int256 output = cheats.envInt(key);
            require(output == expected[i], "envInt failed");
        }
    }

    uint256 constant numEnvAddressTests = 2;

    function testEnvAddress() public {
        string memory key = "_foxarCheatcodeEnvAddressTestKey";
        string[numEnvAddressTests] memory values = [
            "0xcb69fc06a12b7a6f30e2a3c16a3b5d502cd71c20f2f8",
            "0xcb540000000000000000000000000000000000000000"
        ];
        address[numEnvAddressTests] memory expected = [
            0xcb69fc06a12b7a6f30e2a3c16a3b5d502cd71c20f2f8,
            0xcb540000000000000000000000000000000000000000
        ];
        for (uint256 i = 0; i < numEnvAddressTests; ++i) {
            cheats.setEnv(key, values[i]);
            address output = cheats.envAddress(key);
            require(output == expected[i], "envAddress failed");
        }
    }

    uint256 constant numEnvBytes32Tests = 2;

    function testEnvBytes32() public {
        string memory key = "_foxarCheatcodeEnvBytes32TestKey";
        string[numEnvBytes32Tests] memory values = [
            "0xcb69fc06a12b7a6f30e2a3c16a3b5d502cd71c20f2f8",
            "0x00"
        ];
        bytes32[numEnvBytes32Tests] memory expected = [
            bytes32(
                0xcb69fc06a12b7a6f30e2a3c16a3b5d502cd71c20f2f800000000000000000000
            ),
            bytes32(
                0x0000000000000000000000000000000000000000000000000000000000000000
            )
        ];
        for (uint256 i = 0; i < numEnvBytes32Tests; ++i) {
            cheats.setEnv(key, values[i]);
            bytes32 output = cheats.envBytes32(key);
            require(output == expected[i], "envBytes32 failed");
        }
    }

    uint256 constant numEnvStringTests = 2;

    function testEnvString() public {
        string memory key = "_foxarCheatcodeEnvStringTestKey";
        string[numEnvStringTests] memory values = [
            "hello, world!",
            "0xcb69fc06a12b7a6f30e2a3c16a3b5d502cd71c20f2f8"
        ];
        string[numEnvStringTests] memory expected = [
            "hello, world!",
            "0xcb69fc06a12b7a6f30e2a3c16a3b5d502cd71c20f2f8"
        ];
        for (uint256 i = 0; i < numEnvStringTests; ++i) {
            cheats.setEnv(key, values[i]);
            string memory output = cheats.envString(key);
            assertEq(output, expected[i], "envString failed");
        }
    }

    uint256 constant numEnvBytesTests = 2;

    function testEnvBytes() public {
        string memory key = "_foxarCheatcodeEnvBytesTestKey";
        string[numEnvBytesTests] memory values = [
            "0xcb69fc06a12b7a6f30e2a3c16a3b5d502cd71c20f2f8",
            "0x00"
        ];
        bytes[] memory expected = new bytes[](numEnvBytesTests);
        expected[0] = hex"7109709ECfa91a80626fF3989D68f67F5b1DD12D";
        expected[1] = hex"00";
        for (uint256 i = 0; i < numEnvBytesTests; ++i) {
            cheats.setEnv(key, values[i]);
            bytes memory output = cheats.envBytes(key);
            require(
                keccak256(abi.encodePacked((output))) ==
                    keccak256(abi.encodePacked((expected[i]))),
                "envBytes failed"
            );
        }
    }

    function testEnvBoolArr() public {
        string memory key = "_foxarCheatcodeEnvBoolArrTestKey";
        string memory value = "true, false, True, False";
        bool[4] memory expected = [true, false, true, false];

        cheats.setEnv(key, value);
        string memory delimiter = ",";
        bool[] memory output = cheats.envBool(key, delimiter);
        require(
            keccak256(abi.encodePacked((output))) ==
                keccak256(abi.encodePacked((expected))),
            "envBoolArr failed"
        );
    }

    function testEnvUintArr() public {
        string memory key = "_foxarCheatcodeEnvUintArrTestKey";
        string memory value = "0,"
        "115792089237316195423570985008687907853269984665640564039457584007913129639935,"
        "0x0000000000000000000000000000000000000000000000000000000000000000,"
        "0xFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF";
        uint256[4] memory expected = [
            type(uint256).min,
            type(uint256).max,
            type(uint256).min,
            type(uint256).max
        ];

        cheats.setEnv(key, value);
        string memory delimiter = ",";
        uint256[] memory output = cheats.envUint(key, delimiter);
        require(
            keccak256(abi.encodePacked((output))) ==
                keccak256(abi.encodePacked((expected))),
            "envUintArr failed"
        );
    }

    function testEnvIntArr() public {
        string memory key = "_foxarCheatcodeEnvIntArrTestKey";
        string memory value = "-57896044618658097711785492504343953926634992332820282019728792003956564819968,"
        "57896044618658097711785492504343953926634992332820282019728792003956564819967,"
        "-0x8000000000000000000000000000000000000000000000000000000000000000,"
        "0x7FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF";
        int256[4] memory expected = [
            type(int256).min,
            type(int256).max,
            type(int256).min,
            type(int256).max
        ];

        cheats.setEnv(key, value);
        string memory delimiter = ",";
        int256[] memory output = cheats.envInt(key, delimiter);
        require(
            keccak256(abi.encodePacked((output))) ==
                keccak256(abi.encodePacked((expected))),
            "envIntArr failed"
        );
    }

    function testEnvAddressArr() public {
        string memory key = "_foxarCheatcodeEnvAddressArrTestKey";
        string memory value = "0xcb69fc06a12b7a6f30e2a3c16a3b5d502cd71c20f2f8,"
        "0xcb540000000000000000000000000000000000000000";
        address[2] memory expected = [
            0xcb69fc06a12b7a6f30e2a3c16a3b5d502cd71c20f2f8,
            0xcb540000000000000000000000000000000000000000
        ];

        cheats.setEnv(key, value);
        string memory delimiter = ",";
        address[] memory output = cheats.envAddress(key, delimiter);
        require(
            keccak256(abi.encodePacked((output))) ==
                keccak256(abi.encodePacked((expected))),
            "envAddressArr failed"
        );
    }

    function testEnvBytes32Arr() public {
        string memory key = "_foxarCheatcodeEnvBytes32ArrTestKey";
        string memory value = "0xcb69fc06a12b7a6f30e2a3c16a3b5d502cd71c20f2f8,"
        "0x00";
        bytes32[2] memory expected = [
            bytes32(
                0xcb69fc06a12b7a6f30e2a3c16a3b5d502cd71c20f2f800000000000000000000
            ),
            bytes32(
                0x0000000000000000000000000000000000000000000000000000000000000000
            )
        ];

        cheats.setEnv(key, value);
        string memory delimiter = ",";
        bytes32[] memory output = cheats.envBytes32(key, delimiter);
        require(
            keccak256(abi.encodePacked((output))) ==
                keccak256(abi.encodePacked((expected))),
            "envBytes32Arr failed"
        );
    }

    function testEnvStringArr() public {
        string memory key = "_foxarCheatcodeEnvStringArrTestKey";
        string memory value = "hello, world!|"
        "0xcb69fc06a12b7a6f30e2a3c16a3b5d502cd71c20f2f8";
        string[2] memory expected = [
            "hello, world!",
            "0xcb69fc06a12b7a6f30e2a3c16a3b5d502cd71c20f2f8"
        ];

        cheats.setEnv(key, value);
        string memory delimiter = "|";
        string[] memory output = cheats.envString(key, delimiter);
        for (uint256 i = 0; i < expected.length; ++i) {
            require(
                keccak256(abi.encodePacked((output[i]))) ==
                    keccak256(abi.encodePacked((expected[i]))),
                "envStringArr failed"
            );
        }
    }

    function testEnvBytesArr() public {
        string memory key = "_foxarCheatcodeEnvBytesArrTestKey";
        string memory value = "0xcb69fc06a12b7a6f30e2a3c16a3b5d502cd71c20f2f8,"
        "0x00";
        bytes[] memory expected = new bytes[](numEnvBytesTests);
        expected[0] = hex"7109709ECfa91a80626fF3989D68f67F5b1DD12D";
        expected[1] = hex"00";

        cheats.setEnv(key, value);
        string memory delimiter = ",";
        bytes[] memory output = cheats.envBytes(key, delimiter);
        for (uint256 i = 0; i < expected.length; ++i) {
            require(
                keccak256(abi.encodePacked((output[i]))) ==
                    keccak256(abi.encodePacked((expected[i]))),
                "envBytesArr failed"
            );
        }
    }

    function testEnvBoolEmptyArr() public {
        string memory key = "_foxarCheatcodeEnvBoolEmptyArrTestKey";
        string memory value = "";
        bool[] memory expected = new bool[](0);

        cheats.setEnv(key, value);
        string memory delimiter = ",";
        bool[] memory output = cheats.envBool(key, delimiter);
        require(
            keccak256(abi.encodePacked((output))) ==
                keccak256(abi.encodePacked((expected))),
            "envBoolEmptyArr failed"
        );
    }

    function testEnvUintEmptyArr() public {
        string memory key = "_foxarCheatcodeEnvUintEmptyArrTestKey";
        string memory value = "";
        uint256[] memory expected = new uint256[](0);

        cheats.setEnv(key, value);
        string memory delimiter = ",";
        uint256[] memory output = cheats.envUint(key, delimiter);
        require(
            keccak256(abi.encodePacked((output))) ==
                keccak256(abi.encodePacked((expected))),
            "envUintEmptyArr failed"
        );
    }

    function testEnvIntEmptyArr() public {
        string memory key = "_foxarCheatcodeEnvIntEmptyArrTestKey";
        string memory value = "";
        int256[] memory expected = new int256[](0);

        cheats.setEnv(key, value);
        string memory delimiter = ",";
        int256[] memory output = cheats.envInt(key, delimiter);
        require(
            keccak256(abi.encodePacked((output))) ==
                keccak256(abi.encodePacked((expected))),
            "envIntEmptyArr failed"
        );
    }

    function testEnvAddressEmptyArr() public {
        string memory key = "_foxarCheatcodeEnvAddressEmptyArrTestKey";
        string memory value = "";
        address[] memory expected = new address[](0);

        cheats.setEnv(key, value);
        string memory delimiter = ",";
        address[] memory output = cheats.envAddress(key, delimiter);
        require(
            keccak256(abi.encodePacked((output))) ==
                keccak256(abi.encodePacked((expected))),
            "envAddressEmptyArr failed"
        );
    }

    function testEnvBytes32EmptyArr() public {
        string memory key = "_foxarCheatcodeEnvBytes32EmptyArrTestKey";
        string memory value = "";
        bytes32[] memory expected = new bytes32[](0);

        cheats.setEnv(key, value);
        string memory delimiter = ",";
        bytes32[] memory output = cheats.envBytes32(key, delimiter);
        require(
            keccak256(abi.encodePacked((output))) ==
                keccak256(abi.encodePacked((expected))),
            "envBytes32EmptyArr failed"
        );
    }

    function testEnvStringEmptyArr() public {
        string memory key = "_foxarCheatcodeEnvStringEmptyArrTestKey";
        string memory value = "";
        string[] memory expected = new string[](0);

        cheats.setEnv(key, value);
        string memory delimiter = "|";
        string[] memory output = cheats.envString(key, delimiter);
        for (uint256 i = 0; i < expected.length; ++i) {
            require(
                keccak256(abi.encodePacked((output[i]))) ==
                    keccak256(abi.encodePacked((expected[i]))),
                "envStringEmptyArr failed"
            );
        }
    }

    function testEnvBytesEmptyArr() public {
        string memory key = "_foxarCheatcodeEnvBytesEmptyArrTestKey";
        string memory value = "";
        bytes[] memory expected = new bytes[](0);

        cheats.setEnv(key, value);
        string memory delimiter = ",";
        bytes[] memory output = cheats.envBytes(key, delimiter);
        for (uint256 i = 0; i < expected.length; ++i) {
            require(
                keccak256(abi.encodePacked((output[i]))) ==
                    keccak256(abi.encodePacked((expected[i]))),
                "envBytesEmptyArr failed"
            );
        }
    }

    function testEnvOrBoolKey() public {
        string memory key = "_foxarCheatcodeEnvOrBoolTestKey";
        string[numEnvBoolTests] memory values = [
            "true",
            "false",
            "True",
            "False"
        ];
        bool[numEnvBoolTests] memory expected = [true, false, true, false];
        for (uint256 i = 0; i < numEnvBoolTests; ++i) {
            cheats.setEnv(key, values[i]);
            bool output = cheats.envOr(key, expected[i]);
            require(output == expected[i], "envOrBoolKey failed");
        }
    }

    function testEnvOrBoolDefault() public {
        string memory key = "_foxarCheatcodeEnvOrBoolTestDefault";
        bool[numEnvBoolTests] memory expected = [true, false, true, false];
        for (uint256 i = 0; i < numEnvBoolTests; ++i) {
            bool output = cheats.envOr(key, expected[i]);
            require(output == expected[i], "envOrBoolDefault failed");
        }
    }

    function testEnvOrUintKey() public {
        string memory key = "_foxarCheatcodeEnvOrUintTestKey";
        string[numEnvUintTests] memory values = [
            "0",
            "115792089237316195423570985008687907853269984665640564039457584007913129639935",
            "0x01",
            "0x000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000001",
            "0x0000000000000000000000000000000000000000000000000000000000000000",
            "0xFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF"
        ];
        uint256[numEnvUintTests] memory expected = [
            type(uint256).min,
            type(uint256).max,
            1,
            77814517325470205911140941194401928579557062014761831930645393041380819009408,
            type(uint256).min,
            type(uint256).max
        ];
        for (uint256 i = 0; i < numEnvUintTests; ++i) {
            cheats.setEnv(key, values[i]);
            uint256 output = cheats.envOr(key, expected[i]);
            require(output == expected[i], "envOrUintKey failed");
        }
    }

    function testEnvOrUintDefault() public {
        string memory key = "_foxarCheatcodeEnvOrUintTestDefault";
        uint256[numEnvUintTests] memory expected = [
            type(uint256).min,
            type(uint256).max,
            1,
            77814517325470205911140941194401928579557062014761831930645393041380819009408,
            type(uint256).min,
            type(uint256).max
        ];
        for (uint256 i = 0; i < numEnvUintTests; ++i) {
            uint256 output = cheats.envOr(key, expected[i]);
            require(output == expected[i], "envOrUintDefault failed");
        }
    }

    function testEnvOrIntKey() public {
        string memory key = "_foxarCheatcodeEnvOrIntTestKey";
        string[numEnvIntTests] memory values = [
            "-57896044618658097711785492504343953926634992332820282019728792003956564819968",
            "57896044618658097711785492504343953926634992332820282019728792003956564819967",
            "-0x8000000000000000000000000000000000000000000000000000000000000000",
            "0x7FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF"
        ];
        int256[numEnvIntTests] memory expected = [
            type(int256).min,
            type(int256).max,
            type(int256).min,
            type(int256).max
        ];
        for (uint256 i = 0; i < numEnvIntTests; ++i) {
            cheats.setEnv(key, values[i]);
            int256 output = cheats.envOr(key, expected[i]);
            require(output == expected[i], "envOrIntKey failed");
        }
    }

    function testEnvOrIntDefault() public {
        string memory key = "_foxarCheatcodeEnvOrIntTestDefault";
        int256[numEnvIntTests] memory expected = [
            type(int256).min,
            type(int256).max,
            type(int256).min,
            type(int256).max
        ];
        for (uint256 i = 0; i < numEnvIntTests; ++i) {
            int256 output = cheats.envOr(key, expected[i]);
            require(output == expected[i], "envOrIntDefault failed");
        }
    }

    function testEnvOrAddressKey() public {
        string memory key = "_foxarCheatcodeEnvOrAddressTestKey";
        string[numEnvAddressTests] memory values = [
            "0xcb69fc06a12b7a6f30e2a3c16a3b5d502cd71c20f2f8",
            "0xcb540000000000000000000000000000000000000000"
        ];
        address[numEnvAddressTests] memory expected = [
            0xcb69fc06a12b7a6f30e2a3c16a3b5d502cd71c20f2f8,
            0xcb540000000000000000000000000000000000000000
        ];
        for (uint256 i = 0; i < numEnvAddressTests; ++i) {
            cheats.setEnv(key, values[i]);
            address output = cheats.envOr(key, expected[i]);
            require(output == expected[i], "envOrAddressKey failed");
        }
    }

    function testEnvOrAddressDefault() public {
        string memory key = "_foxarCheatcodeEnvOrAddressTestDefault";
        address[numEnvAddressTests] memory expected = [
            0xcb69fc06a12b7a6f30e2a3c16a3b5d502cd71c20f2f8,
            0xcb540000000000000000000000000000000000000000
        ];
        for (uint256 i = 0; i < numEnvAddressTests; ++i) {
            address output = cheats.envOr(key, expected[i]);
            require(output == expected[i], "envOrAddressDefault failed");
        }
    }

    function testEnvOrBytes32Key() public {
        string memory key = "_foxarCheatcodeEnvOrBytes32TestKey";
        string[numEnvBytes32Tests] memory values = [
            "0xcb69fc06a12b7a6f30e2a3c16a3b5d502cd71c20f2f8",
            "0x00"
        ];
        bytes32[numEnvBytes32Tests] memory expected = [
            bytes32(
                0xcb69fc06a12b7a6f30e2a3c16a3b5d502cd71c20f2f800000000000000000000
            ),
            bytes32(
                0x0000000000000000000000000000000000000000000000000000000000000000
            )
        ];
        for (uint256 i = 0; i < numEnvBytes32Tests; ++i) {
            cheats.setEnv(key, values[i]);
            bytes32 output = cheats.envOr(key, expected[i]);
            require(output == expected[i], "envOrBytes32Key failed");
        }
    }

    function testEnvOrBytes32Default() public {
        string memory key = "_foxarCheatcodeEnvOrBytes32TestDefault";
        bytes32[numEnvBytes32Tests] memory expected = [
            bytes32(
                0xcb69fc06a12b7a6f30e2a3c16a3b5d502cd71c20f2f800000000000000000000
            ),
            bytes32(
                0x0000000000000000000000000000000000000000000000000000000000000000
            )
        ];
        for (uint256 i = 0; i < numEnvBytes32Tests; ++i) {
            bytes32 output = cheats.envOr(key, expected[i]);
            require(output == expected[i], "envOrBytes32Default failed");
        }
    }

    function testEnvOrStringKey() public {
        string memory key = "_foxarCheatcodeEnvOrStringTestKey";
        string[numEnvStringTests] memory values = [
            "hello, world!",
            "0xcb69fc06a12b7a6f30e2a3c16a3b5d502cd71c20f2f8"
        ];
        string[numEnvStringTests] memory expected = [
            "hello, world!",
            "0xcb69fc06a12b7a6f30e2a3c16a3b5d502cd71c20f2f8"
        ];
        for (uint256 i = 0; i < numEnvStringTests; ++i) {
            cheats.setEnv(key, values[i]);
            string memory output = cheats.envOr(key, expected[i]);
            assertEq(output, expected[i], "envOrStringKey failed");
        }
    }

    function testEnvOrStringDefault() public {
        string memory key = "_foxarCheatcodeEnvOrStringTestDefault";
        string[numEnvStringTests] memory expected = [
            "hello, world!",
            "0xcb69fc06a12b7a6f30e2a3c16a3b5d502cd71c20f2f8"
        ];
        for (uint256 i = 0; i < numEnvStringTests; ++i) {
            string memory output = cheats.envOr(key, expected[i]);
            assertEq(output, expected[i], "envOrStringDefault failed");
        }
    }

    function testEnvOrBytesKey() public {
        string memory key = "_foxarCheatcodeEnvOrBytesTestKey";
        string[numEnvBytesTests] memory values = [
            "0xcb69fc06a12b7a6f30e2a3c16a3b5d502cd71c20f2f8",
            "0x00"
        ];
        bytes[] memory expected = new bytes[](numEnvBytesTests);
        expected[0] = hex"7109709ECfa91a80626fF3989D68f67F5b1DD12D";
        expected[1] = hex"00";
        for (uint256 i = 0; i < numEnvBytesTests; ++i) {
            cheats.setEnv(key, values[i]);
            bytes memory output = cheats.envOr(key, expected[i]);
            require(
                keccak256(abi.encodePacked((output))) ==
                    keccak256(abi.encodePacked((expected[i]))),
                "envOrBytesKey failed"
            );
        }
    }

    function testEnvOrBytesDefault() public {
        string memory key = "_foxarCheatcodeEnvOrBytesTestDefault";
        bytes[] memory expected = new bytes[](numEnvBytesTests);
        expected[0] = hex"7109709ECfa91a80626fF3989D68f67F5b1DD12D";
        expected[1] = hex"00";
        for (uint256 i = 0; i < numEnvBytesTests; ++i) {
            bytes memory output = cheats.envOr(key, expected[i]);
            require(
                keccak256(abi.encodePacked((output))) ==
                    keccak256(abi.encodePacked((expected[i]))),
                "envOrBytesDefault failed"
            );
        }
    }

    function testEnvOrBoolArrKey() public {
        string memory key = "_foxarCheatcodeEnvBoolWithDefaultBoolArrTestKey";
        string memory value = "true, false, True, False";
        bool[4] memory expected = [true, false, true, false];
        bool[] memory defaultValues = new bool[](4);
        defaultValues[0] = true;
        defaultValues[1] = false;
        defaultValues[2] = true;
        defaultValues[3] = false;

        cheats.setEnv(key, value);
        string memory delimiter = ",";
        bool[] memory output = cheats.envOr(key, delimiter, defaultValues);
        require(
            keccak256(abi.encodePacked((output))) ==
                keccak256(abi.encodePacked((expected))),
            "envOrBoolArrKey failed"
        );
    }

    function testEnvOrBoolArrDefault() public {
        string
            memory key = "_foxarCheatcodeEnvBoolWithDefaultBoolArrTestDefault";
        string memory value = "true, false, True, False";
        bool[4] memory expected = [true, false, true, false];
        bool[] memory defaultValues = new bool[](4);
        defaultValues[0] = true;
        defaultValues[1] = false;
        defaultValues[2] = true;
        defaultValues[3] = false;

        string memory delimiter = ",";
        bool[] memory output = cheats.envOr(key, delimiter, defaultValues);
        require(
            keccak256(abi.encodePacked((output))) ==
                keccak256(abi.encodePacked((expected))),
            "envOrBoolArrDefault failed"
        );
    }

    function testEnvOrUintArrKey() public {
        string memory key = "_foxarCheatcodeEnvOrUintArrTestKey";
        string memory value = "0,"
        "115792089237316195423570985008687907853269984665640564039457584007913129639935,"
        "0x0000000000000000000000000000000000000000000000000000000000000000,"
        "0xFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF";
        uint256[4] memory expected = [
            type(uint256).min,
            type(uint256).max,
            type(uint256).min,
            type(uint256).max
        ];
        uint256[] memory defaultValues = new uint256[](4);
        defaultValues[0] = type(uint256).min;
        defaultValues[1] = type(uint256).max;
        defaultValues[2] = type(uint256).min;
        defaultValues[3] = type(uint256).max;

        cheats.setEnv(key, value);
        string memory delimiter = ",";
        uint256[] memory output = cheats.envOr(key, delimiter, defaultValues);
        require(
            keccak256(abi.encodePacked((output))) ==
                keccak256(abi.encodePacked((expected))),
            "envOrUintArrKey failed"
        );
    }

    function testEnvOrUintArrDefault() public {
        string memory key = "_foxarCheatcodeEnvOrUintArrTestDefault";
        string memory value = "0,"
        "115792089237316195423570985008687907853269984665640564039457584007913129639935,"
        "0x0000000000000000000000000000000000000000000000000000000000000000,"
        "0xFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF";
        uint256[4] memory expected = [
            type(uint256).min,
            type(uint256).max,
            type(uint256).min,
            type(uint256).max
        ];
        uint256[] memory defaultValues = new uint256[](4);
        defaultValues[0] = type(uint256).min;
        defaultValues[1] = type(uint256).max;
        defaultValues[2] = type(uint256).min;
        defaultValues[3] = type(uint256).max;

        string memory delimiter = ",";
        uint256[] memory output = cheats.envOr(key, delimiter, defaultValues);
        require(
            keccak256(abi.encodePacked((output))) ==
                keccak256(abi.encodePacked((expected))),
            "envOrUintArrDefault failed"
        );
    }

    function testEnvOrIntArrKey() public {
        string memory key = "_foxarCheatcodeEnvOrIntArrTestKey";
        string memory value = "-57896044618658097711785492504343953926634992332820282019728792003956564819968,"
        "57896044618658097711785492504343953926634992332820282019728792003956564819967,"
        "-0x8000000000000000000000000000000000000000000000000000000000000000,"
        "0x7FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF";
        int256[4] memory expected = [
            type(int256).min,
            type(int256).max,
            type(int256).min,
            type(int256).max
        ];
        int256[] memory defaultValues = new int256[](4);
        defaultValues[0] = type(int256).min;
        defaultValues[1] = type(int256).max;
        defaultValues[2] = type(int256).min;
        defaultValues[3] = type(int256).max;

        cheats.setEnv(key, value);
        string memory delimiter = ",";
        int256[] memory output = cheats.envOr(key, delimiter, defaultValues);
        require(
            keccak256(abi.encodePacked((output))) ==
                keccak256(abi.encodePacked((expected))),
            "envOrIntArrKey failed"
        );
    }

    function testEnvOrIntArrDefault() public {
        string memory key = "_foxarCheatcodeEnvOrIntArrTestDefault";
        string memory value = "-57896044618658097711785492504343953926634992332820282019728792003956564819968,"
        "57896044618658097711785492504343953926634992332820282019728792003956564819967,"
        "-0x8000000000000000000000000000000000000000000000000000000000000000,"
        "0x7FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF";
        int256[4] memory expected = [
            type(int256).min,
            type(int256).max,
            type(int256).min,
            type(int256).max
        ];
        int256[] memory defaultValues = new int256[](4);
        defaultValues[0] = type(int256).min;
        defaultValues[1] = type(int256).max;
        defaultValues[2] = type(int256).min;
        defaultValues[3] = type(int256).max;

        string memory delimiter = ",";
        int256[] memory output = cheats.envOr(key, delimiter, defaultValues);
        require(
            keccak256(abi.encodePacked((output))) ==
                keccak256(abi.encodePacked((expected))),
            "envOrIntArrDefault failed"
        );
    }

    function testEnvOrAddressArrKey() public {
        string memory key = "_foxarCheatcodeEnvOrAddressArrTestKey";
        string memory value = "0xcb69fc06a12b7a6f30e2a3c16a3b5d502cd71c20f2f8,"
        "0xcb540000000000000000000000000000000000000000";
        address[2] memory expected = [
            0xcb69fc06a12b7a6f30e2a3c16a3b5d502cd71c20f2f8,
            0xcb540000000000000000000000000000000000000000
        ];
        address[] memory defaultValues = new address[](2);
        defaultValues[0] = 0xcb69fc06a12b7a6f30e2a3c16a3b5d502cd71c20f2f8;
        defaultValues[1] = 0xcb540000000000000000000000000000000000000000;

        cheats.setEnv(key, value);
        string memory delimiter = ",";
        address[] memory output = cheats.envOr(key, delimiter, defaultValues);
        require(
            keccak256(abi.encodePacked((output))) ==
                keccak256(abi.encodePacked((expected))),
            "envOrAddressArrKey failed"
        );
    }

    function testEnvOrAddressArrDefault() public {
        string memory key = "_foxarCheatcodeEnvOrAddressArrTestDefault";
        string memory value = "0xcb69fc06a12b7a6f30e2a3c16a3b5d502cd71c20f2f8,"
        "0xcb540000000000000000000000000000000000000000";
        address[2] memory expected = [
            0xcb69fc06a12b7a6f30e2a3c16a3b5d502cd71c20f2f8,
            0xcb540000000000000000000000000000000000000000
        ];
        address[] memory defaultValues = new address[](2);
        defaultValues[0] = 0xcb69fc06a12b7a6f30e2a3c16a3b5d502cd71c20f2f8;
        defaultValues[1] = 0xcb540000000000000000000000000000000000000000;

        string memory delimiter = ",";
        address[] memory output = cheats.envOr(key, delimiter, defaultValues);
        require(
            keccak256(abi.encodePacked((output))) ==
                keccak256(abi.encodePacked((expected))),
            "envOrAddressArrDefault failed"
        );
    }

    function testEnvOrBytes32ArrKey() public {
        string memory key = "_foxarCheatcodeEnvOrBytes32ArrTestKey";
        string memory value = "0xcb69fc06a12b7a6f30e2a3c16a3b5d502cd71c20f2f8,"
        "0x00";
        bytes32[2] memory expected = [
            bytes32(
                0xcb69fc06a12b7a6f30e2a3c16a3b5d502cd71c20f2f800000000000000000000
            ),
            bytes32(
                0x0000000000000000000000000000000000000000000000000000000000000000
            )
        ];
        bytes32[] memory defaultValues = new bytes32[](2);
        defaultValues[0] = bytes32(
            0xcb69fc06a12b7a6f30e2a3c16a3b5d502cd71c20f2f800000000000000000000
        );
        defaultValues[1] = bytes32(
            0x0000000000000000000000000000000000000000000000000000000000000000
        );

        cheats.setEnv(key, value);
        string memory delimiter = ",";
        bytes32[] memory output = cheats.envOr(key, delimiter, defaultValues);
        require(
            keccak256(abi.encodePacked((output))) ==
                keccak256(abi.encodePacked((expected))),
            "envOrBytes32ArrKey failed"
        );
    }

    function testEnvOrBytes32ArrDefault() public {
        string memory key = "_foxarCheatcodeEnvOrBytes32ArrTestDefault";
        string memory value = "0xcb69fc06a12b7a6f30e2a3c16a3b5d502cd71c20f2f8,"
        "0x00";
        bytes32[2] memory expected = [
            bytes32(
                0xcb69fc06a12b7a6f30e2a3c16a3b5d502cd71c20f2f800000000000000000000
            ),
            bytes32(
                0x0000000000000000000000000000000000000000000000000000000000000000
            )
        ];
        bytes32[] memory defaultValues = new bytes32[](2);
        defaultValues[0] = bytes32(
            0xcb69fc06a12b7a6f30e2a3c16a3b5d502cd71c20f2f800000000000000000000
        );
        defaultValues[1] = bytes32(
            0x0000000000000000000000000000000000000000000000000000000000000000
        );

        string memory delimiter = ",";
        bytes32[] memory output = cheats.envOr(key, delimiter, defaultValues);
        require(
            keccak256(abi.encodePacked((output))) ==
                keccak256(abi.encodePacked((expected))),
            "envOrBytes32ArrDefault failed"
        );
    }

    function testEnvOrStringArrKey() public {
        string memory key = "_foxarCheatcodeEnvOrStringArrTestKey";
        string memory value = "hello, world!|"
        "0xcb69fc06a12b7a6f30e2a3c16a3b5d502cd71c20f2f8";
        string[2] memory expected = [
            "hello, world!",
            "0xcb69fc06a12b7a6f30e2a3c16a3b5d502cd71c20f2f8"
        ];
        string[] memory defaultValues = new string[](2);
        defaultValues[0] = "hello, world!";
        defaultValues[1] = "0xcb69fc06a12b7a6f30e2a3c16a3b5d502cd71c20f2f8";

        cheats.setEnv(key, value);
        string memory delimiter = "|";
        string[] memory output = cheats.envOr(key, delimiter, defaultValues);
        for (uint256 i = 0; i < expected.length; ++i) {
            require(
                keccak256(abi.encodePacked((output[i]))) ==
                    keccak256(abi.encodePacked((expected[i]))),
                "envOrStringArrKey failed"
            );
        }
    }

    function testEnvOrStringArrDefault() public {
        string memory key = "_foxarCheatcodeEnvOrStringArrTestDefault";
        string memory value = "hello, world!|"
        "0xcb69fc06a12b7a6f30e2a3c16a3b5d502cd71c20f2f8";
        string[2] memory expected = [
            "hello, world!",
            "0xcb69fc06a12b7a6f30e2a3c16a3b5d502cd71c20f2f8"
        ];
        string[] memory defaultValues = new string[](2);
        defaultValues[0] = "hello, world!";
        defaultValues[1] = "0xcb69fc06a12b7a6f30e2a3c16a3b5d502cd71c20f2f8";

        string memory delimiter = "|";
        string[] memory output = cheats.envOr(key, delimiter, defaultValues);
        for (uint256 i = 0; i < expected.length; ++i) {
            require(
                keccak256(abi.encodePacked((output[i]))) ==
                    keccak256(abi.encodePacked((expected[i]))),
                "envOrStringArrDefault failed"
            );
        }
    }

    function testEnvOrBytesArrKey() public {
        string memory key = "_foxarCheatcodeEnvOrBytesArrTestKey";
        string memory value = "0xcb69fc06a12b7a6f30e2a3c16a3b5d502cd71c20f2f8,"
        "0x00";
        bytes[] memory expected = new bytes[](2);
        expected[0] = hex"7109709ECfa91a80626fF3989D68f67F5b1DD12D";
        expected[1] = hex"00";

        cheats.setEnv(key, value);
        string memory delimiter = ",";
        bytes[] memory output = cheats.envOr(key, delimiter, expected);
        for (uint256 i = 0; i < expected.length; ++i) {
            require(
                keccak256(abi.encodePacked((output[i]))) ==
                    keccak256(abi.encodePacked((expected[i]))),
                "envOrBytesArrKey failed"
            );
        }
    }

    function testEnvOrBytesArrDefault() public {
        string memory key = "_foxarCheatcodeEnvOrBytesArrTestDefault";
        string memory value = "0xcb69fc06a12b7a6f30e2a3c16a3b5d502cd71c20f2f8,"
        "0x00";
        bytes[] memory expected = new bytes[](2);
        expected[0] = hex"7109709ECfa91a80626fF3989D68f67F5b1DD12D";
        expected[1] = hex"00";

        string memory delimiter = ",";
        bytes[] memory output = cheats.envOr(key, delimiter, expected);
        for (uint256 i = 0; i < expected.length; ++i) {
            require(
                keccak256(abi.encodePacked((output[i]))) ==
                    keccak256(abi.encodePacked((expected[i]))),
                "envOrBytesArrDefault failed"
            );
        }
    }

    function testEnvOrBoolEmptyArrKey() public {
        string
            memory key = "_foxarCheatcodeEnvBoolWithDefaultBoolEmptyArrTestKey";
        string memory value = "";
        bool[] memory expected = new bool[](0);
        bool[] memory defaultValues = new bool[](0);

        cheats.setEnv(key, value);
        string memory delimiter = ",";
        bool[] memory output = cheats.envOr(key, delimiter, defaultValues);
        require(
            keccak256(abi.encodePacked((output))) ==
                keccak256(abi.encodePacked((expected))),
            "envOrBoolEmptyArrKey failed"
        );
    }

    function testEnvOrBoolEmptyArrDefault() public {
        string
            memory key = "_foxarCheatcodeEnvBoolWithDefaultBoolEmptyArrTestDefault";
        string memory value = "";
        bool[] memory expected = new bool[](0);
        bool[] memory defaultValues = new bool[](0);

        string memory delimiter = ",";
        bool[] memory output = cheats.envOr(key, delimiter, defaultValues);
        require(
            keccak256(abi.encodePacked((output))) ==
                keccak256(abi.encodePacked((expected))),
            "envOrBoolEmptyArrDefault failed"
        );
    }

    function testEnvOrUintEmptyArrKey() public {
        string memory key = "_foxarCheatcodeEnvOrUintEmptyArrTestKey";
        string memory value = "";
        uint256[] memory expected = new uint256[](0);
        uint256[] memory defaultValues = new uint256[](0);

        cheats.setEnv(key, value);
        string memory delimiter = ",";
        uint256[] memory output = cheats.envOr(key, delimiter, defaultValues);
        require(
            keccak256(abi.encodePacked((output))) ==
                keccak256(abi.encodePacked((expected))),
            "envOrUintEmptyArrKey failed"
        );
    }

    function testEnvOrUintEmptyArrDefault() public {
        string memory key = "_foxarCheatcodeEnvOrUintEmptyArrTestDefault";
        string memory value = "";
        uint256[] memory expected = new uint256[](0);
        uint256[] memory defaultValues = new uint256[](0);

        string memory delimiter = ",";
        uint256[] memory output = cheats.envOr(key, delimiter, defaultValues);
        require(
            keccak256(abi.encodePacked((output))) ==
                keccak256(abi.encodePacked((expected))),
            "envOrUintEmptyArrDefault failed"
        );
    }

    function testEnvOrIntEmptyArrKey() public {
        string memory key = "_foxarCheatcodeEnvOrIntEmptyArrTestKey";
        string memory value = "";
        int256[] memory expected = new int256[](0);
        int256[] memory defaultValues = new int256[](0);

        cheats.setEnv(key, value);
        string memory delimiter = ",";
        int256[] memory output = cheats.envOr(key, delimiter, defaultValues);
        require(
            keccak256(abi.encodePacked((output))) ==
                keccak256(abi.encodePacked((expected))),
            "envOrIntEmptyArrKey failed"
        );
    }

    function testEnvOrIntEmptyArrDefault() public {
        string memory key = "_foxarCheatcodeEnvOrIntEmptyArrTestDefault";
        string memory value = "";
        int256[] memory expected = new int256[](0);
        int256[] memory defaultValues = new int256[](0);

        string memory delimiter = ",";
        int256[] memory output = cheats.envOr(key, delimiter, defaultValues);
        require(
            keccak256(abi.encodePacked((output))) ==
                keccak256(abi.encodePacked((expected))),
            "envOrIntEmptyArrDefault failed"
        );
    }

    function testEnvOrAddressEmptyArrKey() public {
        string memory key = "_foxarCheatcodeEnvOrAddressEmptyArrTestKey";
        string memory value = "";
        address[] memory expected = new address[](0);
        address[] memory defaultValues = new address[](0);

        cheats.setEnv(key, value);
        string memory delimiter = ",";
        address[] memory output = cheats.envOr(key, delimiter, defaultValues);
        require(
            keccak256(abi.encodePacked((output))) ==
                keccak256(abi.encodePacked((expected))),
            "envOrAddressEmptyArrKey failed"
        );
    }

    function testEnvOrAddressEmptyArrDefault() public {
        string memory key = "_foxarCheatcodeEnvOrAddressEmptyArrTestDefault";
        string memory value = "";
        address[] memory expected = new address[](0);
        address[] memory defaultValues = new address[](0);

        string memory delimiter = ",";
        address[] memory output = cheats.envOr(key, delimiter, defaultValues);
        require(
            keccak256(abi.encodePacked((output))) ==
                keccak256(abi.encodePacked((expected))),
            "envOrAddressEmptyArrDefault failed"
        );
    }

    function testEnvOrBytes32EmptyArrKey() public {
        string memory key = "_foxarCheatcodeEnvOrBytes32EmptyArrTestKey";
        string memory value = "";
        bytes32[] memory expected = new bytes32[](0);
        bytes32[] memory defaultValues = new bytes32[](0);

        cheats.setEnv(key, value);
        string memory delimiter = ",";
        bytes32[] memory output = cheats.envOr(key, delimiter, defaultValues);
        require(
            keccak256(abi.encodePacked((output))) ==
                keccak256(abi.encodePacked((expected))),
            "envOrBytes32EmptyArrKey failed"
        );
    }

    function testEnvOrBytes32EmptyArrDefault() public {
        string memory key = "_foxarCheatcodeEnvOrBytes32EmptyArrTestDefault";
        string memory value = "";
        bytes32[] memory expected = new bytes32[](0);
        bytes32[] memory defaultValues = new bytes32[](0);

        string memory delimiter = ",";
        bytes32[] memory output = cheats.envOr(key, delimiter, defaultValues);
        require(
            keccak256(abi.encodePacked((output))) ==
                keccak256(abi.encodePacked((expected))),
            "envOrBytes32EmptyArrDefault failed"
        );
    }

    function testEnvOrStringEmptyArrKey() public {
        string memory key = "_foxarCheatcodeEnvOrStringEmptyArrTestKey";
        string memory value = "";
        string[] memory expected = new string[](0);
        string[] memory defaultValues = new string[](0);

        cheats.setEnv(key, value);
        string memory delimiter = "|";
        string[] memory output = cheats.envOr(key, delimiter, defaultValues);
        for (uint256 i = 0; i < expected.length; ++i) {
            require(
                keccak256(abi.encodePacked((output[i]))) ==
                    keccak256(abi.encodePacked((expected[i]))),
                "envOrStringEmptyArrKey failed"
            );
        }
    }

    function testEnvOrStringEmptyArrDefault() public {
        string memory key = "_foxarCheatcodeEnvOrStringEmptyArrTestDefault";
        string memory value = "";
        string[] memory expected = new string[](0);
        string[] memory defaultValues = new string[](0);

        string memory delimiter = "|";
        string[] memory output = cheats.envOr(key, delimiter, defaultValues);
        for (uint256 i = 0; i < expected.length; ++i) {
            require(
                keccak256(abi.encodePacked((output[i]))) ==
                    keccak256(abi.encodePacked((expected[i]))),
                "envOrStringEmptyArrDefault failed"
            );
        }
    }

    function testEnvWithDefaulBytesEmptyArrKey() public {
        string memory key = "_foxarCheatcodeEnvWithDefaulBytesEmptyArrTestKey";
        string memory value = "";
        bytes[] memory expected = new bytes[](0);
        bytes[] memory defaultValues = new bytes[](0);

        cheats.setEnv(key, value);
        string memory delimiter = ",";
        bytes[] memory output = cheats.envOr(key, delimiter, expected);
        for (uint256 i = 0; i < expected.length; ++i) {
            require(
                keccak256(abi.encodePacked((output[i]))) ==
                    keccak256(abi.encodePacked((expected[i]))),
                "envWithDefaulBytesEmptyArrKey failed"
            );
        }
    }

    function testEnvWithDefaulBytesEmptyArrDefault() public {
        string
            memory key = "_foxarCheatcodeEnvWithDefaulBytesEmptyArrTestDefault";
        string memory value = "";
        bytes[] memory expected = new bytes[](0);
        bytes[] memory defaultValues = new bytes[](0);

        string memory delimiter = ",";
        bytes[] memory output = cheats.envOr(key, delimiter, expected);
        for (uint256 i = 0; i < expected.length; ++i) {
            require(
                keccak256(abi.encodePacked((output[i]))) ==
                    keccak256(abi.encodePacked((expected[i]))),
                "envWithDefaulBytesEmptyArrDefault failed"
            );
        }
    }
}
