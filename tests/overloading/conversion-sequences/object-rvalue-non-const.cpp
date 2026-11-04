// tidal-rules: ICS-40 ICS-13
// tidal-expect: accept
// tidal-note: An rvalue binds the implicit object parameter of get(), which is not const.
template <class T, class U> constexpr bool same = false;
template <class T> constexpr bool same<T, T> = true;
struct One {};
struct Two {};
struct Item { One get(); Two get() const; };
static_assert(same<decltype(Item{}.get()), One>);
