// tidal-rules: DED-37
// tidal-expect: accept
// tidal-note: The pack expansion at the end of List<H, A...> takes each remaining template argument of the argument's specialization, and it can take none.
template <class A, class B> constexpr bool same = false;
template <class A> constexpr bool same<A, A> = true;
template <class... T> struct Tag {};
template <class... T> struct List {};
template <class... A> Tag<A...> all(List<A...>);
template <class H, class... A> Tag<A...> rest(List<H, A...>);
static_assert(same<decltype(all(List<>{})), Tag<>>);
static_assert(same<decltype(all(List<int, char>{})), Tag<int, char>>);
static_assert(same<decltype(rest(List<int>{})), Tag<>>);
static_assert(same<decltype(rest(List<int, char, long>{})), Tag<char, long>>);
