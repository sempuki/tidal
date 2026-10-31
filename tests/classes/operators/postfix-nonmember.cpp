// tidal-rules: OPR-4
// tidal-expect: run
struct C { int v = 0; };
C operator--(C& c, int) { C old = c; --c.v; return old; }
int main() { C c; C old = c--; return old.v == 0 && c.v == -1 ? 0 : 1; }
