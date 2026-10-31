// tidal-rules: OPR-1
// tidal-expect: run
struct S { int operator&() const { return 42; } };
int main() { S s; return &s == 42 ? 0 : 1; }
