// tidal-rules: VIRT-1
// tidal-expect: run
struct B { virtual int f(int) { return 1; } };
struct D : B { int f(long) { return 2; } };
int main() { D d; B& r = d; return r.f(0) == 1 ? 0 : 1; }
