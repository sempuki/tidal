// tidal-rules: INL-3
// tidal-expect: run
// tidal-sources: constexpr-function-inline.tu.cpp
constexpr int sq(int v) { return v * v; }
int other();
int main() { return sq(3) + other() == 18 ? 0 : 1; }
