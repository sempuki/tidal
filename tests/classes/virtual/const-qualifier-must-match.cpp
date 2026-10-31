// tidal-rules: VIRT-1
// tidal-expect: run
struct B { virtual int f() const { return 1; } };
struct D : B { int f() { return 2; } };
int main() { D d; const B& r = d; return r.f() == 1 ? 0 : 1; }
