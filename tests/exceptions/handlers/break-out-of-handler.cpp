// tidal-rules: TRY-15
// tidal-expect: run
int main() { int n = 0; for (int i = 0; i < 3; ++i) { try { throw i; } catch (int v) { n += v; if (v == 1) break; } } return n == 1 ? 0 : 1; }
