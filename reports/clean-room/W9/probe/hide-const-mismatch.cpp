// Probe: a derived visit that misses override because it lacks const on the parameter.
struct A {};
struct V { virtual int visit(const A&) { return 1; } virtual ~V() = default; };
struct C final : V { int visit(A&) { return 2; } };
int main() { C c; V& v = c; A a; return v.visit(a) - 1; }
