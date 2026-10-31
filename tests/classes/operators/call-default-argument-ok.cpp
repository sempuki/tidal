// tidal-rules: OPR-2
// tidal-expect: run
struct S { int operator()(int a = 3) const { return a; } };
int main() { S s; return s() == 3 ? 0 : 1; }
