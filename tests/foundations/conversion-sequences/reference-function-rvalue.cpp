// tidal-rules: ICS-12
// tidal-expect: accept
// tidal-note: An rvalue reference to a function type binds a function lvalue.
template <class T, class U> constexpr bool same = false;
template <class T> constexpr bool same<T, T> = true;
struct One {};
struct Two {};
void run();
One take(void (&&)());
Two take(...);
static_assert(same<decltype(take(run)), One>);
