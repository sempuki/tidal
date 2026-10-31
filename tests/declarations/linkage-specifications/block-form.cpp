// tidal-rules: LSP-1
// tidal-expect: run
extern "C" { int add(int a, int b) { return a + b; } int total = 3; }
int main() { return add(total, 1) == 4 ? 0 : 1; }
