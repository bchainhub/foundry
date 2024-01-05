// SPDX-License-Identifier: Unlicense
pragma solidity 1.1.0;

import "ds-test/test.sol";

// https://github.com/foxar-rs/foxar/issues/3661
contract Issue3661Test is DSTest {
    address sender;

    function setUp() public {
        sender = msg.sender;
    }

    function testSameSender() public {
        assert(sender == msg.sender);
    }
}
