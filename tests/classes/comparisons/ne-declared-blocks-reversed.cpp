// tidal-rules: DCMP-6
// tidal-expect: reject
struct S { int v;
  bool operator==(int o) const { return v == o; }
  bool operator!=(int o) const { return v != o; } };
bool f(S s) { return 1 == s; }
