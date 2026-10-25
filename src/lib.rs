// SPDX-License-Identifier: MIT
pragma solidity ^0.8.24;

/// @title ShortlistRegistry
/// @notice On-chain registry of shortlisted addresses with admin-gated writes.
/// @dev Imported from the monorepo's `contracts/src`. Foundry root is the repo
///      root, so Solidity sources live under `src/` and tests under `test/`.
contract ShortlistRegistry {
    /// @notice Admin allowed to mutate the registry.
    address public admin;

    /// @notice Tracks whether an address is shortlisted.
    mapping(address => bool) public isShortlisted;

    /// @notice Emitted when the admin is set at initialization.
    event Initialized(address indexed admin);
    /// @notice Emitted when an address is added to the shortlist.
    event Shortlisted(address indexed account, address indexed caller);
    /// @notice Emitted when an address is removed from the shortlist.
    event Unshortlisted(address indexed account, address indexed caller);

    error AlreadyInitialized();
    error NotInitialized();
    error Unauthorized();
    error AlreadyShortlisted();
    error NotShortlisted();

    modifier onlyAdmin() {
        if (msg.sender != admin) revert Unauthorized();
        _;
    }

    /// @notice Initialize the contract with an admin address.
    /// @param admin_ The admin allowed to mutate the registry.
    function initialize(address admin_) external {
        if (admin != address(0)) revert AlreadyInitialized();
        admin = admin_;
        emit Initialized(admin_);
    }

    /// @notice Add an address to the shortlist.
    /// @param account The address to shortlist.
    function shortlist(address account) external onlyAdmin {
        if (admin == address(0)) revert NotInitialized();
        if (isShortlisted[account]) revert AlreadyShortlisted();
        isShortlisted[account] = true;
        emit Shortlisted(account, msg.sender);
    }

    /// @notice Remove an address from the shortlist.
    /// @param account The address to remove.
    function unshortlist(address account) external onlyAdmin {
        if (admin == address(0)) revert NotInitialized();
        if (!isShortlisted[account]) revert NotShortlisted();
        isShortlisted[account] = false;
        emit Unshortlisted(account, msg.sender);
    }
}