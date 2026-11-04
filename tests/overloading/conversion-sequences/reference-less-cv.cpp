// tidal-rules: ICS-30
// tidal-expect: accept
// tidal-note: Both bind an lvalue directly by the identity, so only the cv-qualifiers of the referenced types differ.
template <class T, class U> constexpr bool same = false;
template <class T> constexpr bool same<T, T> = true;
struct One {};
struct Two {};
int value = 0;
One take(int&);
Two take(const int&);
static_assert(same<decltype(take(value)), One>);
One look(const int&);
Two look(const volatile int&);
static_assert(same<decltype(look(value)), One>);
