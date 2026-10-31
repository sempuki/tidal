// tidal-rules: TR-5
// tidal-expect: run
constexpr int sq(int v) { return v * v; }
static_assert(sq(3) == 9);
int main() { int n = 3; return sq(n) == 9 ? 0 : 1; }
