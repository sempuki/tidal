// tidal-rules: CTAD-11
// tidal-expect: accept
// tidal-note: The guide comes after the definitions of make and fixed and before their first instantiations, so each deduction in them uses the guide, even the deduction from 1 that does not depend on U. later is instantiated before its guide, so its deduction does not use the guide.
template <class A, class B> constexpr bool same = false;
template <class A> constexpr bool same<A, A> = true;
template <class T> struct Box { constexpr Box(T) {} };
template <class U> constexpr auto make(U u) { Box b(u); return b; }
template <class U> constexpr auto fixed() { Box b(1); return b; }
template <class T> struct Cell { constexpr Cell(T) {} };
template <class U> constexpr auto later(U u) { Cell c(u); return c; }
constexpr auto early = later(1);
Box(int) -> Box<long>;
Cell(int) -> Cell<long>;
constexpr auto x = make(1);
constexpr auto y = fixed<char>();
constexpr auto z = later(1);
static_assert(same<decltype(x), const Box<long>> && same<decltype(y), const Box<long>>);
static_assert(same<decltype(early), const Cell<int>> && same<decltype(z), const Cell<int>>);
