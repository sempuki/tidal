// tidal-rules: OPR-3
// tidal-expect: run
struct S { int operator[](int i = 4) const { return i; } };
int main() { S s; return s[] == 4 && s[1] == 1 ? 0 : 1; }
