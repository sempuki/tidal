// tidal-rules: ICS-29
// tidal-expect: accept
// tidal-note: The argument is a cast to an rvalue reference to a function, which is still a function lvalue. Both parameters bind directly, and only ICS-29 decides.
template <class T, class U> constexpr bool same = false;
template <class T> constexpr bool same<T, T> = true;
struct One {};
struct Two {};
int twice(int);
One take(int (&)(int));
Two take(int (&&)(int));
static_assert(same<decltype(take(static_cast<int (&&)(int)>(twice))), One>);
