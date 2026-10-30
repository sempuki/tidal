// tidal-rules: ACC-11
// tidal-expect: run
struct B { virtual int f() { return 1; } };
class D : public B { int f() override { return 2; } };
int main() { D d; B& b = d; return b.f() == 2 ? 0 : 1; }
