// tidal-rules: ORD-4
// tidal-expect: accept
// tidal-note: The packs are template arguments of List. An invented pack cannot give T a value, and A&... is not deduced from an invented pack of plain types. The last call takes no part in the pack of the third template.
template <class A, class B> constexpr bool same = false;
template <class A> constexpr bool same<A, A> = true;
struct One {}; struct Two {}; struct Three {};
template <class... T> struct List {};
template <class... A> One f(List<A...>);
template <class T, class... A> Two f(List<T, A...>);
template <class T, class... A> Three f(List<T, A&...>);
static_assert(same<decltype(f(List<>{})), One>);
static_assert(same<decltype(f(List<int, float>{})), Two>);
static_assert(same<decltype(f(List<int, float&>{})), Three>);
static_assert(same<decltype(f(List<int>{})), Three>);
