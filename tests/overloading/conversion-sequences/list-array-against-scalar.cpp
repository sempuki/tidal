// tidal-rules: ICS-17
// tidal-expect: accept
// tidal-expect-gcc: reject
// tidal-error-gcc: call of overloaded .take.* is ambiguous
// tidal-note: Q-0135. Both first sequences are the identity, so the second argument decides. GCC prefers int for the first argument and finds the call ambiguous.
template <class T, class U> constexpr bool same = false;
template <class T> constexpr bool same<T, T> = true;
struct One {};
struct Two {};
One take(int, long);
Two take(int (&&)[2], int);
static_assert(same<decltype(take({1}, 0)), Two>);
