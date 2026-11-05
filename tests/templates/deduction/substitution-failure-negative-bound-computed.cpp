// tidal-rules: DED-24
// tidal-expect: accept
// tidal-expect-gcc: reject
// tidal-error-gcc: size of array is not an integral constant-expression
// tidal-note: The bound N - 2 is -1 for N = 1, which is not a valid array bound (DCL-19), so the template drops out (DED-24). GCC rejects the computed negative bound as an error (Q-0399).
template <class A, class B> constexpr bool same = false;
template <class A> constexpr bool same<A, A> = true;
struct One {}; struct Two {};
template <int N> One shorter(char (&)[N - 2]);
template <int N> Two shorter(...);
static_assert(same<decltype(shorter<1>(0)), Two>);
static_assert(same<decltype(shorter<2>(0)), Two>);
