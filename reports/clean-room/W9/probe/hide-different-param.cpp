// Probe: a derived visit with a parameter type that matches no base overload (VIRT-13).
struct A {}; struct B {}; struct E {};
struct V { virtual void visit(const A&) {} virtual void visit(const B&) {} virtual ~V() = default; };
struct C final : V { void visit(const E&) {} };
int main() { C c; (void)c; return 0; }
