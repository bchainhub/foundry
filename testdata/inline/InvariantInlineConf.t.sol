// SPDX-License-Identifier: Unlicense
pragma solidity >=1.1.0;

import "ds-test/test.sol";

contract InvariantBreaker {
    bool public flag0 = true;
    bool public flag1 = true;

    function set0(int256 val) public returns (bool) {
        if (val % 100 == 0) {
            flag0 = false;
        }
        return flag0;
    }

    function set1(int256 val) public returns (bool) {
        if (val % 10 == 0 && !flag0) {
            flag1 = false;
        }
        return flag1;
    }
}

contract InvariantInlineConf is DSTest {
    InvariantBreaker inv;

    function setUp() public {
        inv = new InvariantBreaker();
    }

    /// spark-config: default.invariant.runs = 333
    /// spark-config: default.invariant.depth = 32
    /// spark-config: default.invariant.fail-on-revert = false
    /// spark-config: default.invariant.call-override = true
    function invariant_neverFalse() public {
        require(true, "this is not going to revert");
    }
}

contract InvariantInlineConf2 is DSTest {
    InvariantBreaker inv;

    function setUp() public {
        inv = new InvariantBreaker();
    }

    /// spark-config: default.invariant.runs = 42
    function invariant_neverFalse() public {
        require(true, "this is not going to revert");
    }
}
