// tidal-rules: TRY-6
// tidal-expect: run
int main() { int v = 9; try { throw 1; } catch (int v) { if (v != 1) return 1; } return v == 9 ? 0 : 2; }
