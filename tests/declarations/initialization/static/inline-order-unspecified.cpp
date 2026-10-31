// tidal-rules: SINIT-3
// tidal-expect: run
// tidal-sources: inline-order-unspecified.tu.cpp
// tidal-note: Gap review 2026-10-31.
volatile int seven = 7;
extern int base;
int base = seven;
inline int derived = base + 1;
int other();
int main() { int d = derived; return (d == 8 || d == 1) && other() >= 0 ? 0 : 1; }
