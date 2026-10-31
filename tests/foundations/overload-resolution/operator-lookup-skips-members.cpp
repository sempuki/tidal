// tidal-rules: OVL-7
// tidal-expect: accept
// tidal-note: The member operator== of Panel would hide the non-member in a call by name. The expression a == b still finds the non-member.
template <class T, class U> constexpr bool same = false;
template <class T> constexpr bool same<T, T> = true;
struct One {};
struct Two {};
struct Key {};
One operator==(Key, Key);
struct Panel {
  Two operator==(Panel) const;
  void check(Key a, Key b) { static_assert(same<decltype(a == b), One>); }
};
