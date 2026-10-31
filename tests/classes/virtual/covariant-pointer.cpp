// tidal-rules: VIRT-4
// tidal-expect: run
struct B { virtual B* self() { return this; } int id() { return 1; } };
struct D : B { D* self() override { return this; } int id() { return 2; } };
int main() { D d; return d.self()->id() == 2 ? 0 : 1; }
