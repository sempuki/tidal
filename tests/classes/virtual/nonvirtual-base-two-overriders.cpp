// tidal-rules: VIRT-5
// tidal-expect: run
struct A { virtual int f() { return 1; } };
struct B : A { int f() override { return 2; } };
struct C : A { int f() override { return 3; } };
struct D : B, C {};
int main() { D d; B& b = d; C& c = d; return b.f() == 2 && c.f() == 3 ? 0 : 1; }
