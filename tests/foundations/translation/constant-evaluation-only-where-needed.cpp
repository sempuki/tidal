// tidal-rules: TR-5
// tidal-expect: run
constexpr int f(int d) { return 1 / d; }
int main() { int z = 0; return z == 0 ? 0 : f(z); }
