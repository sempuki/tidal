// tidal-rules: DCMP-6
// tidal-expect: reject
// tidal-error-clang: invalid operands to binary expression
// tidal-error-gcc: no match for .operator==
// tidal-error-edg: no operator .==. matches
struct S { int v;
  bool operator==(int o) const { return v == o; }
  bool operator!=(int o) const { return v != o; } };
bool f(S s) { return 1 == s; }
