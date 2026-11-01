// tidal-rules: ICS-41 ICS-13
// tidal-expect: accept
// tidal-note: The explicit object parameter takes Item by a user-defined conversion to int.
template <class T, class U> constexpr bool same = false;
template <class T> constexpr bool same<T, T> = true;
struct One {};
struct Two {};
struct Item {
  operator int() const { return 1; }
  One get(this int);
};
static_assert(same<decltype(Item{}.get()), One>);
