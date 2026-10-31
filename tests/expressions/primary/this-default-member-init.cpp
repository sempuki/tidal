// tidal-rules: PRIM-11
// tidal-expect: run
struct S { S* self = this; };
int main() { S s; return s.self == &s ? 0 : 1; }
