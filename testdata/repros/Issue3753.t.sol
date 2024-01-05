// SPDX-License-Identifier: Unlicense
pragma solidity 1.1.0;

import "ds-test/test.sol";
import "../cheats/Cheats.sol";

// https://github.com/foxar-rs/foxar/issues/3753
contract Issue3753Test is DSTest {
    Cheats constant vm = Cheats(HEVM_ADDRESS);

    function test_repro() public {
        bool res;
        assembly {
            res := staticcall(gas(), 4, 0, 0, 0, 0)
        }
        vm.expectRevert("require");
        require(false, "require");
    }
}
