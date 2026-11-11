// tidal-rules: LAM-15 CNS-1 OVL-24
// tidal-expect: run
// tidal-note: Roadmap round 11, programmer probe template_lambda_with_head_requires_clause.cpp. The requires clause stands after the template head of the lambda, and it constrains the call operator template (LAM-15, CNS-1). A call with a char argument does not satisfy it, so the call has no viable candidate (OVL-24), and the requires expression in takes is false.
template <class F, class A> constexpr bool takes = requires(F f, A a) { f(a); };
int main() {
  auto widen = []<class T> requires (sizeof(T) > 1) (T x) { return x + 1; };
  static_assert(!takes<decltype(widen), char>);
  static_assert(takes<decltype(widen), int>);
  return widen(41) == 42 ? 0 : 1;
}
