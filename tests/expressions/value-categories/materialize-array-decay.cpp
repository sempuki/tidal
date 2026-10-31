// tidal-rules: VC-8
// tidal-expect: run
using A = int[3];
int sum(const int* p) { return p[0] + p[2]; }
int main() { return sum(A{1, 2, 3}) == 4 ? 0 : 1; }
