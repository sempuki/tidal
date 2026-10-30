// tidal-rules: PFX-6
// tidal-expect: run
int f(int v) { return v; }
int main() { return f(3.9) == 3 ? 0 : 1; }
