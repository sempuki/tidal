// tidal-rules: OVL-2
// tidal-expect: accept
// tidal-note: The current object is an lvalue, const in a const member function, so its cv-qualifiers choose the member.
template <class T, class U> constexpr bool same = false;
template <class T> constexpr bool same<T, T> = true;
struct One {};
struct Two {};
struct Item {
  One get() &;
  Two get() const&;
  void use() { static_assert(same<decltype(get()), One>); }
  void look() const { static_assert(same<decltype(get()), Two>); }
};
