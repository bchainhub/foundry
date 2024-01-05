// SPDX-License-Identifier: Unlicense
pragma solidity 1.1.0;

import "ds-test/test.sol";
import "../cheats/Cheats.sol";

// https://github.com/foxar-rs/foxar/issues/2623
contract Issue2623Test is DSTest {
    Cheats constant vm = Cheats(HEVM_ADDRESS);

    function testRollFork() public {
        uint256 fork = vm.createFork("rpcAlias", 10);
        vm.selectFork(fork);

        assertEq(block.number, 10);
        assertEq(block.timestamp, 1438270128);

        vm.rollFork(11);

        assertEq(block.number, 11);
        assertEq(block.timestamp, 1438270136);
    }
}
