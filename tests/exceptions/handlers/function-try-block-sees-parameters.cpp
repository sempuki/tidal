// tidal-rules: TRY-9
// tidal-expect: run
int f(int p) try { throw 1; } catch (int) { return p; }
int main() { return f(7) == 7 ? 0 : 1; }
