// tidal-rules: CTOR-17
// tidal-expect: accept
// tidal-expect-clang: reject
// tidal-error-clang: binds to a temporary object whose lifetime would be shorter
// tidal-note: Q-0084. Constructor binds a reference member to a temporary; Clang rejects, GCC (with a warning) and EDG accept.
// tidal-note: Gap review 2026-10-31.
struct S { const int& r; S() : r(42) {} };
S s;
