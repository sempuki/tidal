// tidal-rules: UNI-7
// tidal-expect: reject
struct S { union { struct T { int v; }; int i; }; };
