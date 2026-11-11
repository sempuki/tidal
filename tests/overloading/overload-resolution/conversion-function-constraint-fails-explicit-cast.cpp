// tidal-rules: OVL-24 CNS-7
// tidal-expect: run
// tidal-note: Roadmap round 13, programmer reduction conversion_function_unsatisfied_constraint.cpp (from the tries of units.cpp). The conversion function of Amount<false> has a constraint that fails, so the function is not a viable candidate (OVL-24). The cast to int then has no conversion, its substitution in the requires expression is not valid, and the concept is not satisfied (CNS-7).
template <bool Scalar> struct Amount {
  int value = 3;
  constexpr explicit operator int() const requires Scalar { return value; }
};
template <class X> concept ToInt = requires(X x) { static_cast<int>(x); };
int main() {
  if (!ToInt<Amount<true>>) return 1;
  if (ToInt<Amount<false>>) return 2;
  return static_cast<int>(Amount<true>{}) == 3 ? 0 : 3;
}
