// tidal-rules: LSP-4
// tidal-expect: run
extern "C" int f(int x) { return x; }
int f(double) { return 2; }
int main() { return f(1) + f(1.0) == 3 ? 0 : 1; }
