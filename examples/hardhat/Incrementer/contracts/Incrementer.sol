// SPDX-License-Identifier: MIT
pragma solidity ^0.8.4;

contract Incrementer {
    uint256 private _value;
    address private _lastCaller;

    function inc() external {
        _value += 1;
        _lastCaller = msg.sender;
    }

    function add(uint256 delta) external {
        _value += delta;
        _lastCaller = msg.sender;
    }

    function getValue() public view returns ( uint256 ) {
        return _value;
    }
    
    function getLastCaller() public view returns ( address ) {
        return _lastCaller;
    }
}