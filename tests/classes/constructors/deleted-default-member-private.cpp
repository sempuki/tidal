// tidal-rules: CTOR-20
// tidal-expect: reject
// tidal-error-clang: implicitly-deleted default constructor
// tidal-error-gcc: use of deleted function .*S::S\(\)
// tidal-error-edg: default constructor of .S. cannot be referenced|no default constructor exists for class .S|uninitialized const or reference member
class M { M(); };
struct S { M m; };
S s;
