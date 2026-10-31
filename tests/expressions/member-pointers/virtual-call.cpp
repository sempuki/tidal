// tidal-rules: MPTR-3
// tidal-expect: run
struct B { virtual int f() { return 1; } };
struct D : B { int f() override { return 2; } };
int main() { int (B::* pf)() = &B::f; D d; B& b = d; return (b.*pf)() == 2 ? 0 : 1; }
