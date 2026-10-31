// tidal-rules: LSP-1
// tidal-expect: run
extern "C" { extern "C++" { int over(int) { return 1; } int over(double) { return 2; } } }
int main() { return over(1) + over(1.0) == 3 ? 0 : 1; }
