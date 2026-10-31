// tidal-rules: PRIM-12
// tidal-expect: run
struct S { int v = 3; int get() const { return v + this->v; } };
int main() { return S{}.get() == 6 ? 0 : 1; }
