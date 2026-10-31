// tidal-rules: INL-7
// tidal-expect: run
// tidal-sources: inline-in-one-unit-only.tu.cpp
inline int f() { return 1; }
int other();
int main() { return f() + other() == 2 ? 0 : 1; }
