// tidal-rules: INL-7
// tidal-expect: run
// tidal-sources: inline-different-definitions.tu.cpp
inline int pick() { return 1; }
int other();
int main() { int a = pick(), b = other(); return (a == 1 || a == 2) && (b == 1 || b == 2) ? 0 : 1; }
