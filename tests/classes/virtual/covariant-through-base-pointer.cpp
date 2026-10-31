// tidal-rules: VIRT-4
// tidal-expect: run
struct B { int b = 1; virtual B* get() { return this; } };
struct X { int x = 5; };
struct D : X, B { D* get() override { return this; } };
int main() { D d; B& r = d; B* p = r.get(); return p == static_cast<B*>(&d) ? 0 : 1; }
