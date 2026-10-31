// tidal-rules: DCMP-6
// tidal-expect: reject
struct S { int v; int operator==(int o) const { return v == o; } };
bool f(S s) { return s != 1; }
