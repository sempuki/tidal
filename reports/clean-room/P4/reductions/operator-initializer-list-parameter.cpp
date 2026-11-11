// tidal-fe: "unsupported at line 19: overloaded operator". g++ and clang++ run it, exit 0.
// An operator function whose parameter is std::initializer_list<int> (LSUP-1, OPR-6).
namespace std {
template <class E> class initializer_list {
  const E* first;
  decltype(sizeof 0) count;
  constexpr initializer_list(const E* f, decltype(sizeof 0) n) : first(f), count(n) {}
public:
  constexpr const E* begin() const { return first; }
  constexpr const E* end() const { return first + count; }
  constexpr decltype(sizeof 0) size() const { return count; }
};
}
struct L {
  int n;
  bool operator==(std::initializer_list<int> o) const { return o.size() == 2 && n == 1; }
};
int main() {
  L x{1};
  std::initializer_list<int> il = {1, 2};
  return x == il ? 0 : 1;
}
