// tidal-rules: CTOR-1
// tidal-expect: reject
// tidal-error-clang: constructor cannot have a return type
// tidal-error-gcc: return type specification for constructor invalid
// tidal-error-edg: same name as its class must be a constructor
struct S { void S(); };
