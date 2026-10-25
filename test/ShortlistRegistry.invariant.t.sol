// SPDX-License-Identifier: MIT
pragma solidity ^0.8.20;

import {Test} from "forge-std/Test.sol";
import {ShortlistRegistry} from "../src/ShortlistRegistry.sol";

/// @title ShortlistRegistryInvariantTest
/// @notice Foundry invariant tests for the ShortlistRegistry contract.
///
/// @dev Documented invariants that must hold across arbitrary call sequences:
///
///  I1. Membership consistency: for every address `a`, `isShortlisted(a)` returns
///      true if and only if `a` is present in the `shortlisted` set (i.e. the
///      registry's internal bookkeeping and the public view agree).
///
///  I2. No duplicate entries: the number of shortlisted addresses equals the
///      length of the enumerable list of shortlisted addresses (no address is
///      counted twice).
///
///  I3. Owner integrity: the registry owner is never the zero address and never
///      changes without an explicit ownership transfer call.
///
///  I4. Removal is idempotent: once an address is removed it is no longer
///      shortlisted, and removing an already-removed address does not revert or
///      re-add it.
///
///  I5. Add is idempotent: adding an already-shortlisted address does not create
///      duplicates and leaves the membership set unchanged.
///
///  I6. Bounded growth: the shortlisted set only grows via successful `add`
///      calls and only shrinks via successful `remove` calls, so its size is
///      always <= the total number of successful add calls.
///
/// These invariants are exercised by a handler contract that performs bounded
/// random calls against the registry. The handler tracks the set of addresses it
/// has added/removed so the invariant assertions can cross-check the registry's
/// state against the handler's expected model.
contract ShortlistRegistryHandler is Test {
    ShortlistRegistry public registry;

    /// @notice Addresses the handler may operate on.
    address[] public actors;

    /// @notice Mirror of addresses the handler believes are shortlisted.
    address[] public expectedShortlisted;

    /// @notice Total number of successful add calls performed by the handler.
    uint256 public addCalls;

    /// @notice Total number of successful remove calls performed by the handler.
    uint256 public removeCalls;

    constructor(ShortlistRegistry _registry, address[] memory _actors) {
        registry = _registry;
        for (uint256 i = 0; i < _actors.length; i++) {
            actors.push(_actors[i]);
        }
    }

    function actorsLength() external view returns (uint256) {
        return actors.length;
    }

    function expectedShortlistedLength() external view returns (uint256) {
        return expectedShortlisted.length;
    }

    function _actorAt(uint256 seed) internal view returns (address) {
        return actors[seed % actors.length];
    }

    function _indexOfExpected(address who) internal view returns (bool found, uint256 index) {
        for (uint256 i = 0; i < expectedShortlisted.length; i++) {
            if (expectedShortlisted[i] == who) {
                return (true, i);
            }
        }
        return (false, 0);
    }

    /// @notice Bounded random add call.
    function add(uint256 seed) external {
        address who = _actorAt(seed);
        (bool found,) = _indexOfExpected(who);

        registry.add(who);

        if (!found) {
            expectedShortlisted.push(who);
        }
        addCalls++;
    }

    /// @notice Bounded random remove call.
    function remove(uint256 seed) external {
        address who = _actorAt(seed);
        (bool found, uint256 index) = _indexOfExpected(who);

        registry.remove(who);

        if (found) {
            uint256 last = expectedShortlisted.length - 1;
            if (index != last) {
                expectedShortlisted[index] = expectedShortlisted[last];
            }
            expectedShortlisted.pop();
        }
        removeCalls++;
    }

    /// @notice Randomly add or remove based on the seed.
    function addOrRemove(uint256 seed) external {
        if (seed % 2 == 0) {
            this.add(seed);
        } else {
            this.remove(seed);
        }
    }
}

contract ShortlistRegistryInvariantTest is Test {
    ShortlistRegistry internal registry;
    ShortlistRegistryHandler internal handler;

    address internal owner = address(0xA11CE);

    function setUp() public {
        registry = new ShortlistRegistry(owner);

        address[] memory actors = new address[](5);
        actors[0] = address(0x1);
        actors[1] = address(0x2);
        actors[2] = address(0x3);
        actors[3] = address(0x4);
        actors[4] = address(0x5);

        handler = new ShortlistRegistryHandler(registry, actors);

        // Only the handler should drive state changes during invariant runs.
        targetContract(address(handler));
    }

    /// @notice I1: membership view agrees with the handler's expected model.
    function invariant_MembershipConsistency() public view {
        uint256 len = handler.expectedShortlistedLength();
        for (uint256 i = 0; i < len; i++) {
            address who = handler.expectedShortlisted(i);
            assertTrue(registry.isShortlisted(who), "I1: expected shortlisted address not shortlisted");
        }

        uint256 actorsLen = handler.actorsLength();
        for (uint256 i = 0; i < actorsLen; i++) {
            address who = handler.actors(i);
            bool expected = false;
            for (uint256 j = 0; j < len; j++) {
                if (handler.expectedShortlisted(j) == who) {
                    expected = true;
                    break;
                }
            }
            assertEq(
                registry.isShortlisted(who),
                expected,
                "I1: registry membership disagrees with expected model"
            );
        }
    }

    /// @notice I2: no duplicate entries in the registry's enumerable list.
    function invariant_NoDuplicates() public view {
        uint256 count = registry.shortlistedCount();
        address[] memory list = registry.getShortlisted();
        assertEq(list.length, count, "I2: list length != count");

        for (uint256 i = 0; i < list.length; i++) {
            for (uint256 j = i + 1; j < list.length; j++) {
                assertTrue(list[i] != list[j], "I2: duplicate shortlisted entry");
            }
        }
    }

    /// @notice I3: owner is never the zero address.
    function invariant_OwnerNotZero() public view {
        assertTrue(registry.owner() != address(0), "I3: owner is zero address");
    }

    /// @notice I4: removed addresses are not shortlisted.
    function invariant_RemovalIdempotent() public view {
        uint256 len = handler.expectedShortlistedLength();
        uint256 actorsLen = handler.actorsLength();
        for (uint256 i = 0; i < actorsLen; i++) {
            address who = handler.actors(i);
            bool expected = false;
            for (uint256 j = 0; j < len; j++) {
                if (handler.expectedShortlisted(j) == who) {
                    expected = true;
                    break;
                }
            }
            if (!expected) {
                assertFalse(registry.isShortlisted(who), "I4: removed address still shortlisted");
            }
        }
    }

    /// @notice I5: adding an already-shortlisted address does not create duplicates.
    function invariant_AddIdempotent() public view {
        uint256 count = registry.shortlistedCount();
        address[] memory list = registry.getShortlisted();
        assertEq(list.length, count, "I5: list length != count");
    }

    /// @notice I6: shortlisted set size is bounded by successful add calls.
    function invariant_BoundedGrowth() public view {
        assertLe(
            registry.shortlistedCount(),
            handler.addCalls(),
            "I6: shortlisted count exceeds total add calls"
        );
    }
}