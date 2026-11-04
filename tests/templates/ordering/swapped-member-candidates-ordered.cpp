// tidal-rules: ORD-7 ORD-6
// tidal-expect: accept
// tidal-note: Both candidates for w == x are swapped members. Each gets its object parameter and is then reversed, and the second stays more specialized. The case does not depend on Q-0218.
template <class T> struct W {};
struct X {
  template <class T> constexpr bool operator==(const W<T>&) const { return false; }
  template <class T> constexpr bool operator==(const W<T*>&) const { return true; }
};
constexpr X x;
constexpr W<int*> w;
static_assert(w == x);
