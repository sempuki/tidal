// Probe: equality of two pointers to non-virtual member functions in a constant expression.
struct S { void f() {} void g() {} int a; int b; };
static_assert(&S::f != &S::g);
static_assert(&S::a != &S::b);
constexpr void (S::*p)() = &S::f;
static_assert(p == &S::f);
int main() { return 0; }
