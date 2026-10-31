// tidal-rules: OPR-3
// tidal-expect: run
struct S { int operator[]() const { return 5; } };
int main() { S s; return s[] == 5 ? 0 : 1; }
