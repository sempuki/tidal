// Planted mistake: the override differs in const, so it overrides nothing.
struct Base { virtual int f() const { return 1; } virtual ~Base() = default; };
struct Derived : Base { int f() override { return 2; } };
int main() { Derived d; return d.f(); }
