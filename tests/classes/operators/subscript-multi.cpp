// tidal-rules: OPR-3
// tidal-expect: run
struct Grid { int operator[](int r, int c) const { return r * 10 + c; } };
int main() { Grid g; return g[2, 3] == 23 ? 0 : 1; }
