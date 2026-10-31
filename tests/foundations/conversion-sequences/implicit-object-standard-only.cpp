// tidal-rules: ICS-10
// tidal-expect: accept
// tidal-note: A copy into a temporary Item would be a user-defined sequence, so get(int) is not viable for a const object.
template <class T, class U> constexpr bool same = false;
template <class T> constexpr bool same<T, T> = true;
struct One {};
struct Two {};
struct Item { Item(); Item(const Item&); One get(int); Two get(long) const; };
const Item fixed;
static_assert(same<decltype(fixed.get(1)), Two>);
