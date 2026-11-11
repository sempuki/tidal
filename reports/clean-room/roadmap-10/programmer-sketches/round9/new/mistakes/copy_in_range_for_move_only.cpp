struct M { int v; M(int x) : v(x) {} M(const M&) = delete; M(M&&) = default; };
int main() { M a[2] = {M(1), M(2)}; int s = 0; for (auto m : a) s += m.v; return s; }
