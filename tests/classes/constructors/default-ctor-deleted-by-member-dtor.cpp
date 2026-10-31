// tidal-rules: CTOR-3
// tidal-expect: reject
// tidal-error-clang: implicitly-deleted default constructor
// tidal-error-gcc: use of deleted function .*S::S\(\)
// tidal-error-edg: default constructor of .S. cannot be referenced|no default constructor exists for class .S|uninitialized const or reference member
// tidal-note: Gap review 2026-10-31.
struct M { ~M() = delete; };
struct S { M* p; M m; };
S* make() { return new S; }
