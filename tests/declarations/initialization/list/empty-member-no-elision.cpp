// tidal-rules: LIST-13
// tidal-expect: reject
// tidal-error-clang: initializer for aggregate with no elements requires explicit braces
// tidal-error-gcc: initializer for .Empty. must be brace-enclosed
// tidal-error-edg: no suitable constructor exists to convert from .int. to .Empty
// tidal-note: The empty aggregate is the first member, not a base.
struct Empty {};
struct S { Empty e; int v; };
S s{1};
