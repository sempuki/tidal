// tidal-rules: CTOR-2
// tidal-expect: run
struct S { int a = 5; };
int main() { S s; return s.a == 5 ? 0 : 1; }
