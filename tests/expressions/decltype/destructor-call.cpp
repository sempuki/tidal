// tidal-rules: DTY-3
// tidal-expect: accept
// tidal-expect-gcc: reject
// tidal-note: Gap review 2026-10-31. Q-0089. GCC rejects decltype as the type name in an explicit destructor call. Clang and EDG accept it.
struct S { ~S() {} };
void end(S& s) { S t; t.~decltype(t)(); (void)s; }
