// tidal-rules: ICS-9
// tidal-expect: accept
// tidal-note: int to Outer needs two constructors, so no sequence exists.
template <class T, class U> constexpr bool same = false;
template <class T> constexpr bool same<T, T> = true;
struct One {};
struct Two {};
struct Inner { Inner(int); };
struct Outer { Outer(Inner); };
One take(Outer);
Two take(...);
static_assert(same<decltype(take(1)), Two>);
