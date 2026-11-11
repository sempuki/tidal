// tidal-rules: LAM-15 CNS-1 OVL-24
// tidal-expect: run
// tidal-note: Roadmap round 11, programmer reduction template_lambda_with_requires_clause.cpp. The call operator of the lambda is a template with the parameter T (LAM-15), and its trailing requires clause is part of its associated constraints (CNS-1). A call with a char argument does not satisfy the constraint, so the call has no viable candidate (OVL-24), and the requires expression in takes is false.
template <class F, class A> constexpr bool takes = requires(F f, A a) { f(a); };
int main() {
  auto widen = []<class T>(T x) requires (sizeof(T) > 1) { return x + 1; };
  static_assert(!takes<decltype(widen), char>);
  static_assert(takes<decltype(widen), int>);
  return widen(41) == 42 ? 0 : 1;
}
