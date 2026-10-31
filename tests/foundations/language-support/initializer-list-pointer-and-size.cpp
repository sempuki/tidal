// tidal-rules: LSUP-1
// tidal-expect: accept
namespace std {
template <class E> class initializer_list {
  const E* first;
  decltype(sizeof 0) count;
  constexpr initializer_list(const E* f, decltype(sizeof 0) n) : first(f), count(n) {}
public:
  constexpr initializer_list() : first(nullptr), count(0) {}
  constexpr decltype(sizeof 0) size() const { return count; }
  constexpr const E* begin() const { return first; }
  constexpr const E* end() const { return first + count; }
};
}
constexpr int total(std::initializer_list<int> values) {
  int sum = 0;
  for (int v : values) sum += v;
  return sum;
}
static_assert(total({1, 2, 3}) == 6);
