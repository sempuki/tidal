// tidal-rules: UNI-9
// tidal-expect: reject
// tidal-error-clang: types cannot be declared in an anonymous union
// tidal-error-gcc: anonymous union may only have public non-static data members
// tidal-error-edg: types cannot be declared in anonymous unions
struct S { union { struct T { int v; }; int i; }; };
