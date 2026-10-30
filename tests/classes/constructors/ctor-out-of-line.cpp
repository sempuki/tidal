// tidal-rules: CTOR-1
// tidal-expect: run
struct S { S(int); int v; };
S::S(int x) : v(x * 2) {}
int main() { return S(4).v == 8 ? 0 : 1; }
