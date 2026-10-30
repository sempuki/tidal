// tidal-rules: CTOR-5
// tidal-expect: run
struct S { int v; S(int v) : v(v + 1) {} };
int main() { return S(1).v == 2 ? 0 : 1; }
