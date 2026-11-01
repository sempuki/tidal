// tidal-rules: CTAD-5
// tidal-expect: accept
// tidal-note: Each constructor gives a guide. A constructor template adds its own parameters after those of the class, with their defaults, and a pack of the class deduces from a constructor's pack.
template <class A, class B> constexpr bool same = false;
template <class A> constexpr bool same<A, A> = true;
template <class T> struct Ptr { constexpr Ptr(T*) {} template <class U> constexpr Ptr(U*, T) {} };
int n = 0;
Ptr a(&n);
Ptr b(&n, 'c');
static_assert(same<decltype(a), Ptr<int>> && same<decltype(b), Ptr<char>>);
template <class T> struct Opt { template <class U = T> constexpr Opt(T, U = U()) {} };
Opt o(1);
static_assert(same<decltype(o), Opt<int>>);
template <class... T> struct Tuple { constexpr Tuple(T...) {} };
Tuple t(1, 'c', 2.0);
Tuple e;
static_assert(same<decltype(t), Tuple<int, char, double>> && same<decltype(e), Tuple<>>);
