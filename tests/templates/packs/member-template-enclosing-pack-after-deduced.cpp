// tidal-rules: INST-5 DED-1 PACK-1
// tidal-expect: run
// tidal-note: Roadmap round 11, programmer reduction member_template_class_pack_after_deduced_parameter.cpp. Q-0429. The function parameter pack parts expands the pack Ts of the enclosing class template, after the deduced parameter start. The instantiation of Mixer<int, int> makes the member template's declaration with Ts known (INST-5), so parts stands for two int parameters (PACK-1), and deduction finds Total from the first argument (DED-1).
template <class A, class B> constexpr bool same = false;
template <class A> constexpr bool same<A, A> = true;
template <class... Ts> struct Mixer {
  template <class Total> Total sum(Total start, Ts... parts) { return (start + ... + parts); }
};
int main() {
  Mixer<int, int> m;
  long r = m.sum(10L, 2, 3);
  static_assert(same<decltype(m.sum(10L, 2, 3)), long>);
  return r == 15 ? 0 : 1;
}
