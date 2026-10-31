// tidal-rules: ICS-19
// tidal-expect: accept
// tidal-expect-clang-18: reject
// tidal-error-clang-18: call to .take. is ambiguous
// tidal-note: Q-0081. The initializer-list constructor takes {c}, so the sequence to Cell is user-defined and ties with Wrap, and the second argument decides. Clang 18 uses the copy constructor, gives Cell an exact match, and finds the call ambiguous.
namespace std {
template <class E> class initializer_list {
  const E* first_;
  decltype(sizeof 0) size_;
  constexpr initializer_list(const E* f, decltype(sizeof 0) n) : first_(f), size_(n) {}
public:
  constexpr initializer_list() : first_(nullptr), size_(0) {}
};
}
template <class T, class U> constexpr bool same = false;
template <class T> constexpr bool same<T, T> = true;
struct One {};
struct Two {};
struct Cell { Cell(); Cell(std::initializer_list<int>); operator int() const; };
struct Wrap { Wrap(Cell); };
Cell c;
One take(Cell, long);
Two take(Wrap, int);
static_assert(same<decltype(take({c}, 0)), Two>);
