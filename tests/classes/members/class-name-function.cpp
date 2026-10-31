// tidal-rules: CLS-9
// tidal-expect: reject
// tidal-error-clang: constructor cannot have a return type
// tidal-error-gcc: return type specification for constructor invalid
// tidal-error-edg: same name as its class must be a constructor
// tidal-note: Gap review 2026-10-31.
struct S { void S(int); };
