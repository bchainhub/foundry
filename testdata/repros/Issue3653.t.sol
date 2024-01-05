// SPDX-License-Identifier: Unlicense
pragma solidity 1.1.0;

import "ds-test/test.sol";
import "../cheats/Cheats.sol";

// https://github.com/foxar-rs/foxar/issues/3653
contract Issue3653Test is DSTest {
    Cheats constant vm = Cheats(HEVM_ADDRESS);
    uint256 fork;
    Token token;

    constructor() {
        fork = vm.createSelectFork("rpcAlias", 10);
        token = new Token();
        vm.makePersistent(address(token));
    }

    function testDummy() public {
        assertEq(block.number, 10);
    }
}

contract Token {}
