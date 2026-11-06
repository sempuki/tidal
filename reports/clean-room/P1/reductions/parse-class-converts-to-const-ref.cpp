struct E { int e; };
struct R { int v; R(E x) : v(x.e) {} };
int f(const R& r) { return r.v; }
int main() { return f(E{0}); }
