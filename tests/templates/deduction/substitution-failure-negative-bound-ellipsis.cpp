// tidal-rules: DED-24
// tidal-expect: accept
// tidal-expect-edg: reject
// tidal-error-edg: size of an array must be greater than zero
// tidal-note: An explicit argument of -1 makes the parameter a pointer to an array with a bound below one, which is not a valid type (DCL-19), so the template drops out (DED-24). EDG reports the bound as an error when the other overload has an ellipsis (Q-0399).
template <class A, class B> constexpr bool same = false;
template <class A> constexpr bool same<A, A> = true;
struct One {}; struct Two {};
template <int N> One by_pointer(char (*)[N]);
template <int N> Two by_pointer(...);
static_assert(same<decltype(by_pointer<-1>(0)), Two>);
