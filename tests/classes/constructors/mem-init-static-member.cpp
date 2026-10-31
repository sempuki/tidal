// tidal-rules: CTOR-15
// tidal-expect: reject
// tidal-error-clang: does not name a non-static data member or base class
// tidal-error-gcc: is a static data member; it can only be initialized at its definition
// tidal-error-edg: is not a nonstatic data member or base class
struct S { static int n; S() : n(1) {} };
