// tidal-rules: LIST-5
// tidal-expect: reject
struct S { int a; int b; };
S s{.a = 1, .a = 2};
