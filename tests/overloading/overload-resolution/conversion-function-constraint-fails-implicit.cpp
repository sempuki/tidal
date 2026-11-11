// tidal-rules: OVL-24 INIT-7
// tidal-expect: reject
// tidal-error-clang: no viable conversion from
// tidal-error-gcc: cannot convert .Amount<false>. to .int.
// tidal-error-edg: no suitable conversion function from
// tidal-note: Roadmap round 13, programmer probe conversion_function_unsatisfied_constraint_implicit.cpp. The only conversion function of Amount<false> has a constraint that fails, so it is not viable (OVL-24), and the object does not convert to int (INIT-7).
template <bool Scalar> struct Amount {
  int value = 3;
  constexpr operator int() const requires Scalar { return value; }
};
int main() {
  Amount<false> a;
  int n = a;
  return n;
}
