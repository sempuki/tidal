// tidal-rules: DTOR-3
// tidal-expect: reject
struct M { ~M() = delete; };
struct S { M m; };
void f() { S s; }
