// tidal-rules: ICS-13
// tidal-expect: accept
template <class T, class U> constexpr bool same = false;
template <class T> constexpr bool same<T, T> = true;
struct One {};
struct Two {};
struct Item { One get() &; Two get() &&; };
Item i;
static_assert(same<decltype(i.get()), One>);
static_assert(same<decltype(Item{}.get()), Two>);
struct Fixed { One get() const &; Two get() &&; };
static_assert(same<decltype(Fixed{}.get()), Two>);
