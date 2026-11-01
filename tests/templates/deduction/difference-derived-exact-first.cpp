// tidal-rules: DED-15
// tidal-expect: accept
// tidal-note: Rec<int, char> matches Rec<A...> exactly, so the bases are not searched. Its bases Rec<char> and Rec<> would match with other values.
template <class A, class B> constexpr bool same = false;
template <class A> constexpr bool same<A, A> = true;
template <class... T> struct Tag {};
template <class... A> struct Rec;
template <> struct Rec<> {};
template <class H, class... A> struct Rec<H, A...> : Rec<A...> {};
template <class... A> Tag<A...> walk(const Rec<A...>&);
static_assert(same<decltype(walk(Rec<int, char>{})), Tag<int, char>>);
