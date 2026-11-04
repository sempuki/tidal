// tidal-rules: ICS-7
// tidal-expect: accept
// tidal-note: Source to double is the ambiguous sequence. It ties with the user-defined sequence to Wrap, so the second argument decides (overload resolution referenced). chosen-ambiguous-tie.cpp shows the other half.
template <class T, class U> constexpr bool same = false;
template <class T> constexpr bool same<T, T> = true;
struct One {};
struct Two {};
struct Source { operator int(); operator long(); };
struct Wrap { Wrap(Source); };
One take(double, long);
Two take(Wrap, int);
static_assert(same<decltype(take(Source{}, 0)), Two>);
