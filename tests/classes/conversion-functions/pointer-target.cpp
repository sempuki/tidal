// tidal-rules: CVF-1
// tidal-expect: run
struct S { int v = 4; operator int*() { return &v; } };
int main() { S s; int* p = s; return *p == 4 ? 0 : 1; }
