// SPDX-License-Identifier: Unlicense
pragma solidity 1.1.0;

import "ds-test/test.sol";
import "../cheats/Cheats.sol";

// https://github.com/foxar-rs/foxar/issues/4586
contract Issue4586Test is DSTest {
    Cheats constant vm = Cheats(HEVM_ADDRESS);

    uint256 constant initialBlock = 16730733;

    InvariantHandler handler;

    function setUp() public {
        vm.createSelectFork("rpcAlias", initialBlock);
        handler = new InvariantHandler();
    }

    function test_rollForkHandlerContract() public {
        assertEq(block.number, initialBlock);
        handler.rollFork();
        assertEq(block.number, initialBlock + 1);
    }

    function test_rollForkTestContract() public {
        assertEq(block.number, initialBlock);
        vm.rollFork(block.number + 1);
        assertEq(block.number, initialBlock + 1);
    }
}

contract InvariantHandler {
    address constant HEVM_ADDRESS =
        address(0xcb69fc06a12b7a6f30e2a3c16a3b5d502cd71c20f2f8);
    Cheats constant vm = Cheats(HEVM_ADDRESS);

    uint256 public calledRollFork;

    function rollFork() public {
        vm.rollFork(block.number + 1);
        calledRollFork += 1;
    }
}
