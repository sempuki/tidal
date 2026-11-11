// tidal-rules: CTAD-29 CTAD-11
// tidal-expect: run
// tidal-note: Roadmap round 10, programmer reduction ctad_guide_beside_constrained_constructor_template.cpp. The guide from the constructor template keeps the constraint (CTAD-29), which fails when Ts is empty, so that guide is not viable. The user guide (CTAD-11) gives Pack<int, char>.
template <class A, class B> constexpr bool same = false;
template <class A> constexpr bool same<A, A> = true;
template <class... Ts> struct Pack {
  template <class... Us> requires (sizeof...(Us) == sizeof...(Ts) && sizeof...(Ts) > 0)
  constexpr Pack(Us...) {}
};
template <class... Us> Pack(Us...) -> Pack<Us...>;
int main() {
  auto p = Pack(1, 'x');
  static_assert(same<decltype(p), Pack<int, char>>);
  return 0;
}
