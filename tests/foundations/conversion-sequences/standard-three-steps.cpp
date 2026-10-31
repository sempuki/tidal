// tidal-rules: ICS-1
// tidal-expect: accept
// tidal-note: Array to pointer, derived pointer to base pointer, then const added: one step from each group.
template <class T, class U> constexpr bool same = false;
template <class T> constexpr bool same<T, T> = true;
struct One {};
struct Two {};
struct Base {};
struct Derived : Base {};
Derived items[2];
One take(const Base*);
Two take(...);
static_assert(same<decltype(take(items)), One>);
