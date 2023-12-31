// SPDX-License-Identifier: UNLICENSED
pragma solidity ^1.1.0;

contract Counter {
    uint256 public number;

    function setNumber(uint256 newNumber) public {
        number = newNumber;
    }

    function increment() public {
        number++;
    }
}
