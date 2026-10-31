// tidal-rules: ICS-3
// tidal-expect: accept
// tidal-note: The same conversion function serves both overloads, so the reference bindings of the second parts decide (ICS-25, ICS-28).
template <class T, class U> constexpr bool same = false;
template <class T> constexpr bool same<T, T> = true;
struct One {};
struct Two {};
struct Source { operator int(); };
One take(const int&);
Two take(int&&);
static_assert(same<decltype(take(Source{})), Two>);
struct Pointer { operator int*(); };
One look(const int*);
Two look(...);
static_assert(same<decltype(look(Pointer{})), One>);
