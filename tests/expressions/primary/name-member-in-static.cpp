// tidal-rules: PRIM-12
// tidal-expect: reject
// tidal-error-clang: invalid use of member .v. in static member function
// tidal-error-gcc: invalid use of member .S::v. in static member function
// tidal-error-edg: nonstatic member reference must be relative to a specific object
struct S { int v; static int f() { return v; } };
