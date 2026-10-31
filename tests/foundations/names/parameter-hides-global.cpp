// tidal-rules: NAM-2
// tidal-expect: run
int v = 1;
int f(int v) { return v; }
int main() { return f(7) == 7 ? 0 : 1; }
