// SPDX-License-Identifier: Unlicense
pragma solidity 1.1.0;

import "ds-test/test.sol";
import "../cheats/Cheats.sol";
import "../logs/console.sol";

// https://github.com/foxar-rs/foxar/issues/2898
contract Issue2898Test is DSTest {
    address private constant BRIDGE = address(10);
    address private constant BENEFICIARY = address(11);
    Cheats constant vm = Cheats(HEVM_ADDRESS);

    function setUp() public {
        vm.deal(BRIDGE, 100);
        vm.deal(BENEFICIARY, 99);

        vm.setNonce(BRIDGE, 10);
    }

    function testDealBalance() public {
        assertEq(BRIDGE.balance, 100);
        assertEq(BENEFICIARY.balance, 99);
    }

    function testSetNonce() public {
        assertEq(vm.getNonce(BRIDGE), 10);
    }
}
