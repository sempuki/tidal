// tidal-rules: OVL-2
// tidal-expect: accept
// tidal-note: The current object fills the explicit object parameter, so the call by plain name passes one argument.
template <class T, class U> constexpr bool same = false;
template <class T> constexpr bool same<T, T> = true;
struct One {};
struct Two {};
struct Self {
  One take(this Self&, int);
  Two take(this const Self&, long);
  void use() { static_assert(same<decltype(take(1)), One>); }
  void look() const { static_assert(same<decltype(take(1)), Two>); }
};
