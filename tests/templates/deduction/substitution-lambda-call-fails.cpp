// tidal-rules: DED-29 DED-24
// tidal-expect: accept
// tidal-note: The result type calls a generic lambda whose own result type needs x.missing. The call has no viable function, which is a failure of substitution. The template is removed and the fallback is chosen.
template <class A, class B> constexpr bool same = false;
template <class A> constexpr bool same<A, A> = true;
struct One {}; struct Two {};
template <class T>
auto pick(T t) -> decltype([](auto x) -> decltype(x.missing) { return x.missing; }(t), One{});
Two pick(...);
static_assert(same<decltype(pick(1)), Two>);
