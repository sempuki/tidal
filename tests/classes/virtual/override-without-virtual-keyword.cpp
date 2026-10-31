// tidal-rules: VIRT-1
// tidal-expect: run
struct A { virtual int f() { return 1; } };
struct B : A { int f() { return 2; } };
struct C : B { int f() { return 3; } };
int main() { C c; B& r = c; return r.f() == 3 ? 0 : 1; }
