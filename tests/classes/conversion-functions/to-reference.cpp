// tidal-rules: CVF-1
// tidal-expect: run
struct S { int v = 1; operator int&() { return v; } };
int main() { S s; int& r = s; r = 5; return s.v == 5 ? 0 : 1; }
