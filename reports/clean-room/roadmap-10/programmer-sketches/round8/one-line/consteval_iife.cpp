consteval int sq(int x) { return x * x; }
constexpr int t = [] { int s = 0; for (int i = 1; i <= 3; ++i) s += sq(i); return s; }();
static_assert(t == 14);
int main() {}
