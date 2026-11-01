// tidal-rules: CTOR-25
// tidal-expect: reject
// tidal-error-clang: no matching constructor for initialization of .Grid.
// tidal-error-gcc: no matching function for call to .Grid::Grid
// tidal-error-edg: no instance of constructor .Grid::Grid. matches the argument list
// tidal-note: The initializer-list constructor is not viable, so the elements are the arguments, and no constructor takes two Tag.
namespace std {
template <class E> class initializer_list {
  const E* b_;
  decltype(sizeof 0) n_;
  constexpr initializer_list(const E* b, decltype(sizeof 0) n) : b_(b), n_(n) {}
public:
  constexpr initializer_list() : b_(nullptr), n_(0) {}
  constexpr decltype(sizeof 0) size() const { return n_; }
  constexpr const E* begin() const { return b_; }
  constexpr const E* end() const { return b_ + n_; }
};
}
struct Tag {};
struct Grid { Grid(std::initializer_list<int>); Grid(int, int); };
Grid g{Tag{}, Tag{}};
