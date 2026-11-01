// tidal-rules: TSP-9
// tidal-expect: accept
// tidal-note: The partial specialization has two parameters, and the template has one.
template <class T, class U> constexpr bool same = false;
template <class T> constexpr bool same<T, T> = true;
template <class T> struct Split { static constexpr int count = 0; };
template <class R, class A> struct Split<R(A)> { using result = R; using argument = A; static constexpr int count = 1; };
static_assert(Split<int>::count == 0);
static_assert(same<Split<char(long)>::result, char>);
static_assert(same<Split<char(long)>::argument, long>);
