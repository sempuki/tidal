// tidal-rules: LIST-4
// tidal-expect: run
// tidal-note: Gap review 2026-10-31.
struct In { int a, b; };
struct Out { In in; int c; };
int main() { In i{1, 2}; Out o{i, 3}; return o.in.b == 2 && o.c == 3 ? 0 : 1; }
