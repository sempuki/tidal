// tidal-rules: UNI-3
// tidal-expect: reject
struct M { M() {} };
struct S { union { M m; int i; }; };
S s;
