// tidal-rules: INL-3
// tidal-expect: run
// tidal-sources: implicit-special-members-inline.tu.cpp
struct S { int v = 1; };
int other();
int main() { S a; S b = a; return b.v + other() == 2 ? 0 : 1; }
