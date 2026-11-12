struct Base { int f() const { return 1; } };
struct Derived : Base { int f() const override { return 2; } };
int main() { Derived d; return d.f() - 2; }
