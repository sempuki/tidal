// tidal-rules: CTAD-18 CTAD-17
// tidal-expect: accept
// tidal-note: The user guide whose first parameter is std::initializer_list<int> and whose second has a default is an initializer-list guide. It wins a braced list of two in the first phase. With parentheses, only the second phase runs, and (int, int) wins.
namespace std {
template <class E> class initializer_list {
  const E* b_;
  decltype(sizeof 0) n_;
  constexpr initializer_list(const E* b, decltype(sizeof 0) n) : b_(b), n_(n) {}
public:
  constexpr initializer_list() : b_(nullptr), n_(0) {}
};
}
template <class A, class B> constexpr bool same = false;
template <class A> constexpr bool same<A, A> = true;
template <class T> struct Grid {
  constexpr Grid(int, int) {}
  constexpr Grid(std::initializer_list<int>, int = 0) {}
};
Grid(std::initializer_list<int>, int = 0) -> Grid<long>;
Grid(int, int) -> Grid<char>;
Grid braced{1, 2};
Grid parens(1, 2);
static_assert(same<decltype(braced), Grid<long>> && same<decltype(parens), Grid<char>>);
