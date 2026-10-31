// tidal-rules: PRIM-25
// tidal-expect: reject
// tidal-error-clang: invalid use of non-static data member
// tidal-error-gcc: invalid use of non-static data member
// tidal-error-edg: nonstatic member reference must be relative to a specific object
struct S { int v; };
int f() { return S::v; }
