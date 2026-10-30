// tidal-rules: CX-4
// tidal-expect: run
// tidal-sources: function-implicitly-inline.tu.cpp
constexpr int twice(int v) { return v * 2; }
int other();
int main() { return twice(2) == 4 && other() == 6 ? 0 : 1; }
