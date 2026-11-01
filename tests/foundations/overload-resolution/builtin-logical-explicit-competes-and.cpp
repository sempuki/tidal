// tidal-rules: OVL-9 OVL-36
// tidal-expect: reject
// tidal-error-clang: use of overloaded operator .&&. is ambiguous
// tidal-error-gcc: ambiguous overload for .operator&&.
// tidal-error-edg: more than one operator .&&. matches
// tidal-note: The same classes as builtin-logical-explicit-competes.cpp, with &&. The built-in candidate (bool, bool) takes the explicit conversion function, and ties with operator&&(Gate, Gate).
struct Flag { explicit operator bool() const; };
struct Gate { Gate(Flag); };
int operator&&(Gate, Gate);
void use(Flag f) { f && f; }
