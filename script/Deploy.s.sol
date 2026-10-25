// SPDX-License-Identifier: MIT
pragma solidity ^0.8.13;

import {Script} from "forge-std/Script.sol";
import {ShortlistRegistry} from "../src/ShortlistRegistry.sol";

/// @title Deploy
/// @notice Deployment script for the ShortlistRegistry contract.
/// @dev Credentials are never hardcoded or committed. Provide the deployer key
///      via the PRIVATE_KEY environment variable, or use a keystore account:
///
///        forge script script/Deploy.s.sol:Deploy \
///          --rpc-url $RPC_URL \
///          --account <keystore-account-name> \
///          --broadcast
///
///      When using --account, the PRIVATE_KEY environment variable is not
///      required; Foundry resolves the signer from the keystore.
contract Deploy is Script {
    /// @notice Deploys the ShortlistRegistry contract.
    /// @return registry The deployed ShortlistRegistry instance.
    function run() external returns (ShortlistRegistry registry) {
        // Read the deployer private key from the environment. This is only
        // used when broadcasting with a raw key; when running with
        // `--account`, Foundry supplies the signer and this value is unused.
        uint256 deployerPrivateKey = vm.envOr("PRIVATE_KEY", uint256(0));

        vm.startBroadcast(deployerPrivateKey);

        registry = new ShortlistRegistry();

        vm.stopBroadcast();
    }
}