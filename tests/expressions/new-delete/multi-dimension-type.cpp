// tidal-rules: NEW-2
// tidal-expect: run
// tidal-note: Gap review 2026-10-31.
int main() { int n = 2; int (*p)[3] = new int[n][3]; p[1][2] = 5; int v = p[1][2]; delete[] p; return v == 5 ? 0 : 1; }
