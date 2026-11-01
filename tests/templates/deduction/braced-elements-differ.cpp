// tidal-rules: DED-16 DED-35
// tidal-expect: reject
// tidal-error-clang: deduced conflicting types for parameter
// tidal-error-gcc: no matching function for call to
// tidal-error-edg: no instance of function template
// tidal-note: The elements give T the values int and long.
namespace std {
template <class E> class initializer_list {
  const E* b_;
  decltype(sizeof 0) n_;
  constexpr initializer_list(const E* b, decltype(sizeof 0) n) : b_(b), n_(n) {}
public:
  constexpr initializer_list() : b_(nullptr), n_(0) {}
  constexpr decltype(sizeof 0) size() const { return n_; }
};
}
template <class T> void list(std::initializer_list<T>);
void use() { list({1, 2L}); }
