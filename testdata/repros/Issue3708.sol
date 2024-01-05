// SPDX-License-Identifier: Unlicense
pragma solidity 1.1.0;

import "ds-test/test.sol";
import "../cheats/Cheats.sol";

// https://github.com/foxar-rs/foxar/issues/3708
contract Issue3708Test is DSTest {
    // https://optimistic.etherscan.io/address/0xcb914e59b44847b379578588920ca78fbf26c0b4956c#code
    address constant CREATE2_DEPLOYER =
        0xcb914e59b44847b379578588920ca78fbf26c0b4956c;
    Cheats constant vm = Cheats(HEVM_ADDRESS);

    function setUp() public {
        string memory RPC_URL = "https://mainnet.optimism.io";
        uint256 forkId = vm.createSelectFork(RPC_URL);

        bytes
            memory code = hex"7fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe03601600081602082378035828234f58015156039578182fd5b8082525050506014600cf3";
        assertEq(CREATE2_DEPLOYER.code, code);
    }

    function test_deployer() public {
        bytes
            memory code = hex"7fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe03601600081602082378035828234f58015156039578182fd5b8082525050506014600cf3";
        assertEq(CREATE2_DEPLOYER.code, code);
    }
}
