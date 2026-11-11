// tidal-rules: INST-1 INST-2
// tidal-expect: run
// tidal-note: Roadmap round 13, programmer probe decltype_qualifier_operator_template_conversion_template.cpp. The operand of decltype is a call of an operator function template. The qualified name after decltype names a member of the result class, and that use instantiates the class (INST-1).
template <int N> struct Level {
  static constexpr int depth = N;
  constexpr explicit Level(int) {}
};
template <int A, int B> constexpr Level<A - B> operator/(Level<A>, Level<B>) { return Level<A - B>(0); }
static_assert(decltype(Level<4>(0) / Level<1>(0))::depth == 3);
int main() { return decltype(Level<2>(0) / Level<2>(0))::depth; }
