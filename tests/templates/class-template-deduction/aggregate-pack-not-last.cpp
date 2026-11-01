// tidal-rules: CTAD-27 CTAD-9
// tidal-expect: accept
// tidal-note: The pack of bases B comes before the member x, so the pack matches no element of the list and is empty. The one value initializes x.
template <class A, class B> constexpr bool same = false;
template <class A> constexpr bool same<A, A> = true;
template <class... B> struct Tail : B... { int x; };
Tail t{5};
static_assert(same<decltype(t), Tail<>>);
