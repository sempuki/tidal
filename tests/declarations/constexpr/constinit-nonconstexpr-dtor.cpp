// tidal-rules: CINIT-1
// tidal-expect: accept
// tidal-note: Gap review 2026-10-31.
struct S { constexpr S() {} ~S() {} };
constinit S s;
