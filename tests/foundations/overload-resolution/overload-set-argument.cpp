// tidal-rules: OVL-39
// tidal-expect: accept
// tidal-note: Each g takes a pointer to function. Only the pairing of g(void (*)(int)) with f(int) matches, so that g wins.
template <class T, class U> constexpr bool same = false;
template <class T> constexpr bool same<T, T> = true;
struct One {};
struct Two {};
void f(int);
void f(double);
One g(void (*)(int));
Two g(void (*)(char));
static_assert(same<decltype(g(f)), One>);
