// Inheriting constructors with a member that has its own initializer.
struct Base { int a, b; Base(int x) : a(x), b(0) {} Base(int x, int y) : a(x), b(y) {} };
struct Derived : Base { using Base::Base; int c = 7; };
int main() { Derived d(2, 3); Derived e(4); return d.a + d.b + d.c + e.a + e.b == 16 ? 0 : 1; }
