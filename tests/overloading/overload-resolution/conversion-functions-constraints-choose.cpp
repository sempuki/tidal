// tidal-rules: OVL-24 CNS-7
// tidal-expect: run
// tidal-note: Roadmap round 13, programmer probe conversion_functions_two_constraints.cpp. The two conversion functions differ only in their constraints. In each specialization one constraint fails, so that conversion function is not viable (OVL-24) and the other one converts.
template <bool Exact> struct Reading {
  int raw = 9;
  constexpr explicit operator int() const requires Exact { return raw; }
  constexpr explicit operator int() const requires (!Exact) { return raw / 2; }
};
int main() {
  if (static_cast<int>(Reading<true>{}) != 9) return 1;
  return static_cast<int>(Reading<false>{}) == 4 ? 0 : 2;
}
