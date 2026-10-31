// tidal-rules: OPR-4
// tidal-expect: run
struct C { int v = 0;
  C& operator++() { ++v; return *this; }
  C operator++(int) { C old = *this; ++v; return old; } };
int main() { C c; C old = c++; ++c; return old.v == 0 && c.v == 2 ? 0 : 1; }
