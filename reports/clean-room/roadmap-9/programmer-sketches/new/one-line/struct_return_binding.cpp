struct MinMax { int lo, hi; };
constexpr MinMax minmax(const int (&a)[4]) { MinMax m{a[0], a[0]}; for (int x : a) { if (x < m.lo) m.lo = x; if (x > m.hi) m.hi = x; } return m; }
int main() { int a[4] = {3, -1, 7, 2}; auto [lo, hi] = minmax(a); return lo == -1 && hi == 7 ? 0 : 1; }
