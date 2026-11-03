// tidal-rules: OVL-32
// tidal-expect: accept
// tidal-note: Write report W7. For a == b, the friend from the class of a is a normal candidate, and the friend from the class of b is a reversed one. The normal candidate wins, in either operand order.
template <int N> struct Label {
  int id;
  template <int M>
  friend constexpr int operator==(const Label& a, const Label<M>& b) { return N * 10 + M + (a.id == b.id ? 100 : 0); }
};
constexpr Label<2> small{7};
constexpr Label<8> large{7};
static_assert((small == large) == 128);   // from Label<2>
static_assert((large == small) == 182);   // from Label<8>
