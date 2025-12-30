# TLA+ Walkthrough: Authenticated Session Model

We have successfully modeled the authentication flow of `hello-actix` using TLA+. This process not
only documented the system but also revealed how TLA+ catches subtle bugs like Session ID
collisions and Resource Exhaustion.

## 1. The Model

The specification at [specs/SessionModel.tla](../specs/SessionModel.tla) defines:

- **Users**: A set of users (e.g., u1, u2).
- **Server**: Stores active sessions (`SessionID -> User`).
- **Client**: Stores cookies locally (`{User, SessionID}`).
- **Actions**: `Login`, `Logout`, `Expire`, `ClientClearsCookies`.

## 2. Verification Journey

### A. The "Session Integrity" Bug (Found by TLC)

Initially, our `Login` action blindly picked any empty session slot. TLC immediately found a trace
violating the **Safety Invariant**:

1. **u1** logs in, gets Session 1.
1. **u1** logs out. Server clears Session 1. **Client u1 still keeps the cookie**.
1. **u2** logs in. Server reuses Session 1.
1. **Result**: Client u1 now has a valid cookie for Session 1, which belongs to **u2**. This is a
   **Session Reuse Vulnerability**.

**Fix**: We updated `Login` to ensure the server never reuses a Session ID that is currently held
by *any* client (simulating fresh, random UUIDs).

### B. The Deadlock (Resource Exhaustion)

After fixing the security bug, TLC reported a **Deadlock**.

1. Users kept logging in and logging out.
1. Because `Login` was forced to pick "Fresh" IDs, we eventually ran out of IDs (since we modeled a
   finite set `1..3`).
1. Once all IDs were "tainted" by old cookies, no one could log in. The system halted.

**Fix**: We added `ClientClearsCookies`, allowing the system to recover resources and run
indefinitely.

## 3. How to Run Verification

We have included the TLA+ tools in the `specs/` directory.

### Run the Model Checker

```bash
cd specs
java -cp tla2tools.jar tlc2.TLC SessionModel.tla
```

**Expected Output**:

```text
Model checking completed. No error has been found.
...
1765 states generated...
```

## 4. Key Takeaways

- **Safety**: TLA+ proved we must generate unique Session IDs (e.g., UUIDs) and not reuse simple
  integers.
- **Liveness**: Infinite operation requires a mechanism to clean up state (Cookie
  expiration/clearing), or an infinite ID space (sufficiently large random numbers).
