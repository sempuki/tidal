// tidal-rules: OVL-7
// tidal-expect: accept
// tidal-note: Each expression has one best candidate of a different kind: a member, a non-member, and a built-in operator after a conversion.
template <class T, class U> constexpr bool same = false;
template <class T> constexpr bool same<T, T> = true;
struct One {};
struct Two {};
struct Num {
  One operator+(int) const;
  operator long() const;
};
Two operator+(const Num&, double);
Num n;
static_assert(same<decltype(n + 1), One>);
static_assert(same<decltype(n + 1.0), Two>);
static_assert(same<decltype(n * 1), long>);
