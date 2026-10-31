// tidal-rules: CVF-2
// tidal-expect: run
struct S { operator int() const { return 3; } };
int main() { int* p = new int[S{}]; p[2] = 1; delete[] p; return 0; }
