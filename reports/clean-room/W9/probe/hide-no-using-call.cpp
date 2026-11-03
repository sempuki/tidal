// Probe: the same, with a direct call of the hidden overload (DER-19).
struct A {}; struct B {};
struct V { virtual void visit(const A&) = 0; virtual void visit(const B&) = 0; protected: ~V() = default; };
struct W : V { void visit(const A&) override {} void visit(const B&) override {} };
struct C final : W { int n = 0; void visit(const A&) override { ++n; } };
int main() { C c; B b; c.visit(b); return 0; }
