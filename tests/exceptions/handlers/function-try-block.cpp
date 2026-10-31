// tidal-rules: TRY-9
// tidal-expect: run
int f() try { throw 1; } catch (int v) { return v + 1; }
int main() { return f() == 2 ? 0 : 1; }
