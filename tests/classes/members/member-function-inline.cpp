// tidal-rules: CLS-4
// tidal-expect: run
// tidal-sources: member-function-inline.tu.cpp
struct S { int twice(int v) const { return v * 2; } };
int other();
int main() { return S{}.twice(2) == 4 && other() == 6 ? 0 : 1; }
