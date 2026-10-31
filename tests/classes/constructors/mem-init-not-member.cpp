// tidal-rules: CTOR-15
// tidal-expect: reject
// tidal-error-clang: does not name a non-static data member or base class
// tidal-error-gcc: does not have any field named .b
// tidal-error-edg: is not a nonstatic data member or base class
struct S { int a; S() : b(1) {} };
