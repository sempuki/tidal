// tidal-rules: ICS-29
// tidal-expect: accept
// tidal-note: Both bind directly to the same function type, and a function type has no cv-qualifiers. Only ICS-29 decides.
template <class T, class U> constexpr bool same = false;
template <class T> constexpr bool same<T, T> = true;
struct One {};
struct Two {};
void run();
One take(void (&)());
Two take(void (&&)());
static_assert(same<decltype(take(run)), One>);
