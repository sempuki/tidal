// tidal-rules: CTOR-18
// tidal-expect: reject
// tidal-expect-edg: accept
// tidal-note: Q-0083. Default member initializer of a nested class used before the enclosing class is complete; Clang and GCC reject, EDG accepts.
// tidal-note: Gap review 2026-10-31.
struct Outer { struct In { int v = 1; }; Outer(In = {}) {} };
Outer o;
