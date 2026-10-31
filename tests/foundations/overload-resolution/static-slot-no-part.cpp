// tidal-rules: OVL-25
// tidal-expect: accept
// tidal-note: The static member's object slot takes no part, so the int argument decides.
template <class T, class U> constexpr bool same = false;
template <class T> constexpr bool same<T, T> = true;
struct One {};
struct Two {};
struct Item {
  static One get(int);
  Two get(long) const;
};
Item v;
static_assert(same<decltype(v.get(1)), One>);
static_assert(same<decltype(v.get(1L)), Two>);
