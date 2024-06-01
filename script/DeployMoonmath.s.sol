// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

import "forge-std/Script.sol";
import "../src/Moonmath.sol";

contract DeployMoonmath is Script {
    function run() external {
        vm.startBroadcast();
        new Moonmath();
        vm.stopBroadcast();
    }
}

