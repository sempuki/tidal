// tidal-rules: LSP-4
// tidal-expect: run
// tidal-sources: c-cross-unit.tu.cpp
extern "C" int helper(int);
int main() { return helper(3) == 9 ? 0 : 1; }
