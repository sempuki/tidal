// tidal-rules: ICS-17
// tidal-expect: accept
// tidal-note: Each element converts to int by the identity, an extra element of a known bound comes from empty braces, and two elements do not fit a bound of one.
template <class T, class U> constexpr bool same = false;
template <class T> constexpr bool same<T, T> = true;
struct One {};
struct Two {};
One take(int (&&)[1]);
Two take(long);
static_assert(same<decltype(take({1})), One>);
One open(int (&&)[]);
Two open(...);
static_assert(same<decltype(open({1, 2})), One>);
One room(int (&&)[3]);
Two room(long);
static_assert(same<decltype(room({1, 2})), One>);
One text(const char (&)[3]);
Two text(...);
static_assert(same<decltype(text({"ab"})), One>);
One full(int (&&)[1]);
Two full(int (&&)[2]);
static_assert(same<decltype(full({1, 2})), Two>);
