// tidal-rules: CTAD-6 CTAD-1
// tidal-expect: accept
// tidal-note: Tag declares no constructor, so a guide with no parameters gives Tag<T> and T takes its default.
template <class A, class B> constexpr bool same = false;
template <class A> constexpr bool same<A, A> = true;
template <class T = int> struct Tag {};
Tag a;
Tag b{};
static_assert(same<decltype(a), Tag<int>> && same<decltype(b), Tag<int>>);
