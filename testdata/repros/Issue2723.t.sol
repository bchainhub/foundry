// SPDX-License-Identifier: Unlicense
pragma solidity 1.1.0;

import "ds-test/test.sol";
import "../cheats/Cheats.sol";

// https://github.com/foxar-rs/foxar/issues/2723
contract Issue2723Test is DSTest {
    Cheats constant vm = Cheats(HEVM_ADDRESS);

    function testRollFork() public {
        address coinbase = 0xcb740193d941b50d91be6567c7ee1c0fe7af498b4137;

        vm.createSelectFork("rpcAlias", 9);

        assertEq(block.number, 9);
        assertEq(coinbase.balance, 11250000000000000000);

        vm.rollFork(10);

        assertEq(block.number, 10);
        assertEq(coinbase.balance, 16250000000000000000);
    }
}
