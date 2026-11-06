struct E { int e; };
struct R { int v; R(E x) : v(x.e) {} };
int main() { E e{0}; const R& r = e; return r.v; }
