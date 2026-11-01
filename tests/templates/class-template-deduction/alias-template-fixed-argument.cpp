// tidal-rules: CTAD-14
// tidal-expect: accept
// tidal-expect-clang-18: reject
// tidal-error-clang-18: requires template arguments; argument deduction only allowed for class templates
// tidal-expect-edg: reject
// tidal-error-edg: cannot deduce class template arguments
// tidal-note: IntPair fixes the first argument of Pair, and an alias of an alias works by the same steps. Clang 18 and EDG 7.0 reject both (Q-0240).
template <class A, class B> constexpr bool same = false;
template <class A> constexpr bool same<A, A> = true;
template <class T, class U> struct Pair { constexpr Pair(T, U) {} };
template <class U> using IntPair = Pair<int, U>;
IntPair p(1L, 'c');
static_assert(same<decltype(p), Pair<int, char>>);
template <class T, class U> using Both = Pair<T, U>;
template <class U> using LongFirst = Both<long, U>;
LongFirst q(1, 'c');
static_assert(same<decltype(q), Pair<long, char>>);
