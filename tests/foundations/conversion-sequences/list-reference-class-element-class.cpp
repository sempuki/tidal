// tidal-rules: ICS-20
// tidal-expect: accept
// tidal-expect-gcc: reject
// tidal-error-gcc: static assertion failed
// tidal-note: Q-0157. Source is not Item or derived from Item, so REF-14 makes a temporary Item from {s}, and Item& cannot bind it. GCC binds Item& through operator Item&.
template <class T, class U> constexpr bool same = false;
template <class T> constexpr bool same<T, T> = true;
struct One {};
struct Two {};
struct Item { Item(); Item(const Item&); };
struct Source { operator Item&(); };
Source s;
One edit(Item&);
Two edit(const Item&);
static_assert(same<decltype(edit({s})), Two>);
