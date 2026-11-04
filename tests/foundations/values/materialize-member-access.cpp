// tidal-rules: VC-8
// tidal-expect: run
struct S { int v = 7; };
int main() { return S{}.v == 7 ? 0 : 1; }
