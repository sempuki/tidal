// Probe: a derived visitor overrides one overload of visit and omits the using-declaration.
struct A; struct B;
struct V { virtual void visit(const A&) = 0; virtual void visit(const B&) = 0; protected: ~V() = default; };
struct W : V { void visit(const A&) override {} void visit(const B&) override {} };
struct C final : W { int n = 0; void visit(const A&) override { ++n; } };
int main() { C c; (void)c; return 0; }
