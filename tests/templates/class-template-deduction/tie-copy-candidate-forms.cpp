// tidal-rules: CTAD-21
// tidal-expect: accept
// tidal-note: In copy-initialization and in a braced initializer, the copy candidate and the guide from the copy constructor tie. The copy candidate wins, and the uses are not ambiguous.
template <class A, class B> constexpr bool same = false;
template <class A> constexpr bool same<A, A> = true;
template <class T> struct Copy { constexpr Copy(T) {} constexpr Copy(const Copy&) {} };
constexpr Copy<char> one('c');
Copy two = one;
Copy three{one};
static_assert(same<decltype(two), Copy<char>> && same<decltype(three), Copy<char>>);
