// tidal-rules: VC-8
// tidal-expect: run
using Pair = int[2];
int sum(const int* p) { return p[0] + p[1]; }
int main() { return sum(Pair{4, 5}) == 9 ? 0 : 1; }
