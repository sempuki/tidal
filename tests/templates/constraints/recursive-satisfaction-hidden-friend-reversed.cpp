// tidal-rules: CNS-35 CNS-11
// tidal-expect: reject
// tidal-error-clang: depends on itself
// tidal-error-gcc: depends on itself
// tidal-expect-edg: accept
// tidal-expect-clang-18: accept
// tidal-note: Roadmap round 15, programmer reduction recursive_constraint_hidden_friend_mixin.cpp (from strong_first_try.cpp). Q-0453. For Meters(4) * 2, the candidate operator*(K, const Meters&) deduces K = Meters, and its constraint needs d.value() * k for a long long and a Meters. That expression considers operator*(const Meters&, K) with K = Meters, whose first parameter cannot take the long long. CNS-11 checks the constraint of that candidate anyway, and the constraint needs the same expression for a long long and a Meters again. So the check of the constraint of operator*(const Meters&, K) with K = Meters needs its own result, which CNS-35 rejects. GCC 14 and 16 and Clang 22 reject the program. EDG 7.0 and Clang 18 accept it.
template <class D> struct Scaled {
  template <class K>
    requires requires(const D& d, K k) { d.value() * k; }
  friend constexpr D operator*(const D& d, K k) { return D(d.value() * k); }
  template <class K>
    requires requires(const D& d, K k) { d.value() * k; }
  friend constexpr D operator*(K k, const D& d) { return D(d.value() * k); }
};
struct Meters : Scaled<Meters> {
  long long v;
  constexpr explicit Meters(long long x) : v(x) {}
  constexpr long long value() const { return v; }
};
constexpr Meters scaled = Meters(4) * 2;
