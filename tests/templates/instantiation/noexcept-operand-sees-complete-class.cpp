// tidal-rules: INST-30 CLS-2
// tidal-expect: accept
// tidal-note: RV10-R-1. Holder<Coin> in the operand of a noexcept specifier inside Coin has its point right after Coin's definition, so Holder<Coin> sees the member d that Coin declares after the operand.
template <class T> struct Holder {
  T value;
  static constexpr int size = sizeof(T);
};
struct Coin {
  int c;
  void f() noexcept(Holder<Coin>::size == 2 * sizeof(int));
  int d;
};
static_assert(noexcept(Coin{}.f()));
