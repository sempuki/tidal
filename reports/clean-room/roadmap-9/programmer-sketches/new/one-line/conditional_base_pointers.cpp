struct B { virtual ~B() = default; virtual int f() const { return 0; } };
struct D1 : B { int f() const override { return 1; } };
struct D2 : B { int f() const override { return 2; } };
int main(int argc, char**) { D1 a; D2 b; const B* p = argc > 0 ? static_cast<const B*>(&a) : &b; return p->f() == 1 ? 0 : 1; }
