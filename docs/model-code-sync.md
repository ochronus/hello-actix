# TLA+ Spec and Code Synchronization

This guide explains how to maintain the relationship between our formal specifications in `specs/`
and the application code.

## Determine Scope & Traceability

The TLA+ specifications are valid only if the code honors the logical constraints defined in the
model. We use specific comments in the code to link implementation details back to the formal
model.

### Annotations

Search for `[TLA+` in the codebase to find these links.

- `[TLA+ Action]`: Indicates a function or block of code that implements a specific TLA+ Action
  (e.g., `Login`, `Logout`).
- `[TLA+ Invariant]`: Indicates code that enforces a Safety Invariant (e.g., generating unique
  IDs).

**Example**:

```rust
// [TLA+ Action] SessionModel!Login
pub async fn login(...) { ... }
```

## The Workflow

### 1. When Changing Critical Logic

If you are modifying authentication flows, session handling, or distributed state logic:

1. **Check the Spec**: Look at `specs/SessionModel.tla`. Does your change violate an invariant
   (e.g., reusing IDs)?
1. **Update the Spec**: If you are adding a new feature (e.g., "Force Logout All Devices"), model
   it in TLA+ first.
1. **Verify**: Run `java -cp tla2tools.jar tlc2.TLC SessionModel.tla` in the `specs/` directory.
1. **Implement**: Write the code, ensuring it matches the new model constraints.

### 2. When Refactoring

If you are just refactoring code (renaming variables, optimizing) without changing logic:

- Ensure the `[TLA+ ...]` comments remain attached to the relevant logic.

## Verification Checklist

- [ ] **Freshness**: Does the Session Store use random IDs? (Yes -> Satisfies `Login` safety).
- [ ] **Cleanup**: Do we have expiration? (Yes -> Satisfies `Expire` liveness).
- [ ] **Integrity**: Is it possible for a user to access a session they don't own? (Middleware
      prevents this).
