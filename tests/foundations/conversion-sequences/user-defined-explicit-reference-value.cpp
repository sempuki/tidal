// tidal-rules: ICS-45
// tidal-expect: accept
// tidal-expect-clang: reject
// tidal-error-clang: static assertion failed
// tidal-note: Q-0156. Direct initialization of a reference does not use an explicit conversion function that returns a value. Clang uses it.
template <class T, class U> constexpr bool same = false;
template <class T> constexpr bool same<T, T> = true;
struct One {};
struct Two {};
template <class T, class S> concept makes = requires(S s) { static_cast<T>(s); };
struct Item { Item(); };
struct ToItem { explicit operator Item(); };
struct ToInt { explicit operator int(); };
static_assert(!makes<const Item&, ToItem>);
static_assert(!makes<Item&&, ToItem>);
static_assert(!makes<int&&, ToInt>);
