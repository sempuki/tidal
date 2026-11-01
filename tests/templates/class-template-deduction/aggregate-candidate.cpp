// tidal-rules: CTAD-9
// tidal-expect: accept
// tidal-note: Each element of the list deduces against the declared type of the member that it initializes, in braces, in parentheses, and with designators. A later member of type T deduces T again.
template <class A, class B> constexpr bool same = false;
template <class A> constexpr bool same<A, A> = true;
template <class T, class U> struct Pair { T first; U second; };
Pair a{1, 'c'};
Pair b(1, 2.0);
Pair c{.first = 1L, .second = 2u};
static_assert(same<decltype(a), Pair<int, char>> && same<decltype(b), Pair<int, double>>);
static_assert(same<decltype(c), Pair<long, unsigned>>);
template <class T> struct Twice { T x; T y; };
Twice t{1, 2};
static_assert(same<decltype(t), Twice<int>>);
