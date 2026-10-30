// tidal-rules: DEL-4
// tidal-expect: accept
// tidal-note: Gap review 2026-10-30.
struct S {
  int v;
  friend bool operator==(const S&, const S&) = default;
};
bool same_value(S a, S b) { return a == b; }
