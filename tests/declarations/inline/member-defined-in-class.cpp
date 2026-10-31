// tidal-rules: INL-3
// tidal-expect: run
// tidal-sources: member-defined-in-class.tu.cpp
struct S { int get() const { return 3; } };
int main() { return S{}.get() == 3 ? 0 : 1; }
