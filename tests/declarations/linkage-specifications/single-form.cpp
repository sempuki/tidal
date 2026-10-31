// tidal-rules: LSP-1
// tidal-expect: run
extern "C" int mul(int a, int b) { return a * b; }
int main() { return mul(2, 3) == 6 ? 0 : 1; }
