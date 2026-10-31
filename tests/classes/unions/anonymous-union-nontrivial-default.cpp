// tidal-rules: UNI-3
// tidal-expect: reject
// tidal-error-clang: implicitly-deleted default constructor of .S
// tidal-error-gcc: use of deleted function .*S::S\(\)
// tidal-error-edg: default constructor of .S. cannot be referenced
struct M { M() {} };
struct S { union { M m; int i; }; };
S s;
