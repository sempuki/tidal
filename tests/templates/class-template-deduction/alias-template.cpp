// tidal-rules: CTAD-14
// tidal-expect: accept
// tidal-expect-clang-18: reject
// tidal-error-clang-18: requires template arguments; argument deduction only allowed for class templates
// tidal-note: The guides of Pair reach Same through the alias, with the alias's own parameters. Clang 18 has no deduction through an alias template (Q-0240).
template <class A, class B> constexpr bool same = false;
template <class A> constexpr bool same<A, A> = true;
template <class T, class U> struct Pair { constexpr Pair(T, U) {} };
template <class T> using Same = Pair<T, T>;
Same s(1, 2);
static_assert(same<decltype(s), Pair<int, int>>);
