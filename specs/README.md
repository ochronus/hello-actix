# TLA+ Specifications for hello-actix

This directory contains formal specifications for critical parts of the application.

## Models

### SessionModel

Models the authentication lifecycle including:

- Login (Session creation)
- Logout (Session invalidation)
- Session Expiration
- Cookie/Server state synchronization issues

## How to Run

Reference: `SessionModel.tla`

We have included `tla2tools.jar` (The standard TLA+ CLI tools) in this directory for convenience.

To verify the model:

```bash
java -cp tla2tools.jar tlc2.TLC SessionModel.tla
```

Use `-deadlock` if you want to find deadlocks (though this model is designed to run indefinitely so
it might deadlock if state space is exhausted without a loop, but here users can always login/out).

## Files

- `SessionModel.tla`: The specification source.
- `SessionModel.cfg`: Configuration for the model checker (number of users, etc).
