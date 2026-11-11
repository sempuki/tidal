// tidal-rules: INST-5 DED-1 PACK-1 DED-20
// tidal-expect: run
// tidal-note: Roadmap round 11, programmer reduction member_template_class_pack_before_deduced_parameter.cpp. Q-0429. The function parameter pack parts expands the pack Ts of the enclosing class template. The instantiation of Mixer<int, int> makes the declaration of the member template with Ts known (INST-5), so parts stands for two int parameters (PACK-1). Deduction then finds only Total from the last argument (DED-1).
template <class A, class B> constexpr bool same = false;
template <class A> constexpr bool same<A, A> = true;
template <class... Ts> struct Mixer {
  template <class Total> Total sum(Ts... parts, Total start) { return (start + ... + parts); }
};
int main() {
  Mixer<int, int> m;
  long r = m.sum(2, 3, 10L);
  static_assert(same<decltype(m.sum(2, 3, 10L)), long>);
  return r == 15 ? 0 : 1;
}
