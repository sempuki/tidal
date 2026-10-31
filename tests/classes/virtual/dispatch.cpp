// tidal-rules: VIRT-1
// tidal-expect: run
struct B { virtual int f() { return 1; } };
struct D : B { int f() { return 2; } };
int main() { D d; B& r = d; B* p = &d; return r.f() == 2 && p->f() == 2 ? 0 : 1; }
