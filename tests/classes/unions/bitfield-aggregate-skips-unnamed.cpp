// tidal-rules: BF-7
// tidal-expect: run
struct S { int a : 4; int : 4; int b : 4; };
int main() { S s{1, 2}; return s.a == 1 && s.b == 2 ? 0 : 1; }
