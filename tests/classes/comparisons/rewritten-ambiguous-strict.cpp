// tidal-rules: OPR-15
// tidal-expect: reject
// tidal-error-clang: to be ambiguous
// tidal-error-gcc: these are ambiguous, even though the second is reversed
// tidal-error-edg: more than one operator .==. matches these operands
// tidal-note: Without -Wall, -pedantic-errors makes the warning of Clang and GCC an error. EDG reports the ambiguity as an error.
struct S { int v; bool operator==(const S& o) { return v == o.v; } };
bool f(S a, S b) { return a == b; }
