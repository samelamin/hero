// SPDX-License-Identifier: GPL-3.0-only
pragma solidity >=0.8.0;

contract Incrementor {
    uint private value;
    address private last_caller;

    function inc() public {
        value += 1;
        last_caller = msg.sender;
    }

    function add(uint delta) public {
        value += delta;
        last_caller = msg.sender;
    }

    function get_value() view public returns (uint) {
        return value;
    }

    function get_last_caller() view public returns (address) {
        return last_caller;
    }
}