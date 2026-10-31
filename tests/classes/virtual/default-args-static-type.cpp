// tidal-rules: VIRT-1
// tidal-expect: run
struct B { virtual int f(int x = 1) { return x; } };
struct D : B { int f(int x = 2) override { return x * 10; } };
int main() { D d; B& r = d; return r.f() == 10 && d.f() == 20 ? 0 : 1; }
