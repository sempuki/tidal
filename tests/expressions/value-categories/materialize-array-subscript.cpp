// tidal-rules: VC-8
// tidal-expect: run
using A = int[3];
int main() { return A{1, 2, 3}[1] == 2 ? 0 : 1; }
