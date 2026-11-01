// tidal-rules: DED-4
// tidal-expect: accept
// tidal-note: The plain function is the exact match for show(1). With <> the plain function is not a candidate, so the template is called, with T from its default.
template <class A, class B> constexpr bool same = false;
template <class A> constexpr bool same<A, A> = true;
struct One {}; struct Two {};
One show(int);
template <class T = int> Two show(long);
static_assert(same<decltype(show(1)), One>);
static_assert(same<decltype(show<>(1)), Two>);
