// tidal-rules: TRY-6
// tidal-expect: run
int main() { try { throw 1; } catch (int v) { { int v = 5; if (v != 5) return 1; } return v == 1 ? 0 : 2; } return 3; }
