// tidal-rules: DCL-8
// tidal-expect: run
int f(int x);
int f(const int x) { return x + 1; }
int main() { return f(1) == 2 ? 0 : 1; }
