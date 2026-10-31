// tidal-rules: OPR-3
// tidal-expect: run
struct S { static int operator[](int i) { return i + 1; } };
int main() { S s; return s[1] == 2 ? 0 : 1; }
