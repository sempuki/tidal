// tidal-rules: NAM-6
// tidal-expect: run
struct S { int get() const { return v; } int v = 9; };
int main() { return S{}.get() == 9 ? 0 : 1; }
