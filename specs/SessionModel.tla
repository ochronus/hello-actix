-------------------------- MODULE SessionModel --------------------------
EXTENDS Integers, Sequences, FiniteSets, TLC

CONSTANT Users          \* Set of User IDs
CONSTANT MaxSessions    \* Max active sessions to limit state space

VARIABLE serverSessions \* Function: SessionID -> UserID (or NULL)
VARIABLE clientCookies  \* Set of {user, sessionID} records visible to clients

Vars == <<serverSessions, clientCookies>>

\* Session IDs are integers for simplicity
SessionIDs == 1..MaxSessions

TypeOK ==
    /\ serverSessions \in [SessionIDs -> Users \cup {"NULL"}]
    /\ clientCookies \in SUBSET [u: Users, s: SessionIDs]

Init ==
    /\ serverSessions = [s \in SessionIDs |-> "NULL"]
    /\ clientCookies = {}

-----------------------------------------------------------------------------

\* Action: User u logs in and gets assigned session s
Login(u) ==
    \E s \in SessionIDs:
        /\ serverSessions[s] = "NULL"  \* Find empty slot
        /\ \A c \in clientCookies: c.s /= s \* Security Fix: Ensure ID is fresh (not held by any client)
        /\ serverSessions' = [serverSessions EXCEPT ![s] = u]
        /\ clientCookies' = clientCookies \cup {[u |-> u, s |-> s]}

\* Action: User u logs out (clears session on server)
\* Note: Cookie remains on client but becomes invalid on server
Logout(u) ==
    \E c \in clientCookies:
        /\ c.u = u
        /\ serverSessions[c.s] = u     \* Only if currently valid
        /\ serverSessions' = [serverSessions EXCEPT ![c.s] = "NULL"]
        /\ UNCHANGED <<clientCookies>> \* Client keeps the cookie!

\* Action: Session expires (cleared from server by middleware/time)
ExpireSession ==
    \E s \in SessionIDs:
        /\ serverSessions[s] /= "NULL"
        /\ serverSessions' = [serverSessions EXCEPT ![s] = "NULL"]
        /\ UNCHANGED <<clientCookies>>

\* Action: Access a protected resource
\* This is just a state check - if we can execute this, the access is successful.
AccessProtected(u) ==
    \E c \in clientCookies:
        /\ c.u = u
        /\ serverSessions[c.s] = u \* Verification: Cookie must match server Session
    /\ UNCHANGED Vars

\* Action: Client clears their cookies (e.g. browser clear data)
ClientClearsCookies ==
    \E c \in clientCookies:
        /\ clientCookies' = clientCookies \ {c}
        /\ UNCHANGED serverSessions

-----------------------------------------------------------------------------

Next ==
    \/ \E u \in Users: Login(u)
    \/ \E u \in Users: Logout(u)
    \/ ExpireSession
    \/ ClientClearsCookies


\* Invariant: No session leakage
\* If a session on the server belongs to User A, then if that session ID is in a client cookie,
\* it should belong to User A (not User B trying to hijack).
SessionIntegrity ==
    \A c \in clientCookies:
        (serverSessions[c.s] /= "NULL") => (serverSessions[c.s] = c.u)

\* Liveness property (optional for now, just example)
\* If a user is logged in, they eventually log out or expire
\* Spec == Init /\ [][Next]_Vars

=============================================================================
