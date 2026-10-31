// tidal-rules: UNEV-1 PRIM-25
// tidal-expect: reject
// tidal-error-clang: use of non-static data member .m. of .O. from nested type
// tidal-error-gcc: invalid use of non-static data member
// tidal-error-edg: nonstatic member reference must be relative to a specific object
struct O { int m; struct I { int g() { return m; } }; };
