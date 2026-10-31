// tidal-rules: TYN-7
// tidal-expect: reject
// tidal-error-clang: no struct named .Missing. in .Outer
// tidal-error-gcc: qualified name does not name a class
// tidal-error-edg: has no tag member named .Missing
struct Outer { struct Inner; };
struct Inner2 {};
struct Outer::Missing { int v; };
