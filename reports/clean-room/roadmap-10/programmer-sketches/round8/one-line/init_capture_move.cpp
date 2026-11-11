struct M { int v; M(int x) : v(x) {} M(M&& o) : v(o.v) { o.v = 0; } };
int main() { M m(5); auto f = [n = static_cast<M&&>(m)] { return n.v; }; return f() == 5 && m.v == 0 ? 0 : 1; }
