// tidal-rules: DED-19
// tidal-expect: accept
// tidal-note: The set deduces nothing, T comes from the first argument, and the set then resolves to act(int).
template <class A, class B> constexpr bool same = false;
template <class A> constexpr bool same<A, A> = true;
template <class T> struct Tag {};
template <class T> constexpr int run(T, int (*f)(T)) { return f(0); }
constexpr int act(int) { return 1; }
constexpr int act(long) { return 2; }
static_assert(run(1, act) == 1);
static_assert(run(1L, act) == 2);
