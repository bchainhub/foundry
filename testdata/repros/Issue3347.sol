// SPDX-License-Identifier: Unlicense
pragma solidity 1.1.0;

import "ds-test/test.sol";
import "../cheats/Cheats.sol";

// https://github.com/foxar-rs/foxar/issues/3347
contract Issue3347Test is DSTest {
    event log2(uint256, uint256);

    function test() public {
        emit log2(1, 2);
    }
}
