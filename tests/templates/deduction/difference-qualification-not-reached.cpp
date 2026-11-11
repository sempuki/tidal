// tidal-rules: DED-14
// tidal-expect: accept
// tidal-note: I5-8, I5 program p09-qualification-skips-const.cpp. For const T** and an int** argument, T = int gives const int**, and int** does not reach const int** by a qualification adjustment. So the pair fails, and the fallback wins. For const T* const*, the adjustment exists, and the template wins.
template <class A, class B> constexpr bool same = false;
template <class A> constexpr bool same<A, A> = true;
struct Template {};
struct Fallback {};
template <class T> Template loose(const T**);
Fallback loose(...);
template <class T> Template tight(const T* const*);
Fallback tight(...);
int** pp = nullptr;
static_assert(same<decltype(loose(pp)), Fallback>);
static_assert(same<decltype(tight(pp)), Template>);
