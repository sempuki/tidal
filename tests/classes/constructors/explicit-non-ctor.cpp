// tidal-rules: CTOR-9
// tidal-expect: reject
// tidal-error-clang: explicit. can only be applied to a constructor or conversion function
// tidal-error-gcc: only declarations of constructors and conversion operators can be .explicit
// tidal-error-edg: explicit. is not allowed
struct S { explicit void f(); };
