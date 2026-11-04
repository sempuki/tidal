// tidal-rules: ICS-12
// tidal-expect: accept
template <class T, class U> constexpr bool same = false;
template <class T> constexpr bool same<T, T> = true;
struct One {};
struct Two {};
int value = 0;
One change(int&);
Two change(...);
static_assert(same<decltype(change(1)), Two>);
One steal(int&&);
Two steal(...);
static_assert(same<decltype(steal(value)), Two>);
One watch(volatile const int&);
Two watch(...);
static_assert(same<decltype(watch(1)), Two>);
