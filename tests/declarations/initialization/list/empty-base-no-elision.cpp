// tidal-rules: LIST-13
// tidal-expect: reject
// tidal-error-clang: initializer for aggregate with no elements requires explicit braces
// tidal-error-gcc: initializer for .Empty. must be brace-enclosed
// tidal-error-edg: no suitable constructor exists to convert from .int. to .Empty
// tidal-note: Gap review 2026-10-31.
struct Empty {};
struct S : Empty { int v; };
S s{1};
