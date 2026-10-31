// tidal-rules: VIRT-6
// tidal-expect: run
struct B { virtual int f() { return 1; } };
struct M { int v; M(B& b) : v(b.f()) {} };
struct D : B { M m; D() : m(*this) {} int f() override { return 2; } };
int main() { D d; return d.m.v == 2 ? 0 : 1; }
