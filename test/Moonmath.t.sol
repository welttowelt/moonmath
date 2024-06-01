// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

import "forge-std/Test.sol";
import "../src/Moonmath.sol";

contract MoonmathTest is Test {
    Moonmath moonmath;

    function setUp() public {
        moonmath = new Moonmath();
    }

    function testPerformInteraction() public {
        moonmath.performInteraction{value: 0.00001 ether}();
    }
}

