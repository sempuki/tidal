// tidal-rules: LSUP-1
// tidal-expect: reject
// tidal-expect-edg: accept
// tidal-error-clang: layout not recognized|unexpected layout
// tidal-error-gcc: does not match
// tidal-note: P2-6. The class declares the count before the pointer, so its shape is not the shape of LSUP-1, and a braced list cannot make an object of it. EDG accepts the class (Q-0431).
namespace std {
template <class E> class initializer_list {
  decltype(sizeof 0) count;
  const E* first;
  constexpr initializer_list(const E* f, decltype(sizeof 0) n) : count(n), first(f) {}
public:
  constexpr initializer_list() : count(0), first(nullptr) {}
  constexpr decltype(sizeof 0) size() const { return count; }
};
}
constexpr int length(std::initializer_list<int> values) { return static_cast<int>(values.size()); }
static_assert(length({1, 2, 3}) == 3);
