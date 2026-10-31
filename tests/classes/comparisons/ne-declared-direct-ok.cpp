// tidal-rules: DCMP-6
// tidal-expect: accept
struct S { int v;
  bool operator==(int o) const { return v == o; }
  bool operator!=(int o) const { return v != o; } };
bool f(S s) { return s == 1 && s != 2; }
