// tidal-rules: OPR-6
// tidal-expect: run
enum E { a = 1, b = 2 };
int main() { int x = a + b; return x == 3 ? 0 : 1; }
