// tidal-rules: OPR-6 LSUP-1
// tidal-expect: run
// tidal-note: P4-2. Tally has a member operator== whose parameter is std::initializer_list<int>, declared in the shape of LSUP-1. In t == parts, the member candidates of == include that function (OPR-6), and the named operand matches the parameter exactly. GCC, Clang, and EDG accept.
namespace std {
template <class E> class initializer_list {
  const E* items;
  decltype(sizeof 0) length;
  constexpr initializer_list(const E* p, decltype(sizeof 0) n) : items(p), length(n) {}
public:
  constexpr const E* begin() const { return items; }
  constexpr decltype(sizeof 0) size() const { return length; }
};
}
struct Tally {
  int total;
  bool operator==(std::initializer_list<int> parts) const {
    int sum = 0;
    for (decltype(sizeof 0) i = 0; i < parts.size(); ++i) sum += parts.begin()[i];
    return sum == total;
  }
};
int main() {
  Tally t{6};
  std::initializer_list<int> parts = {1, 2, 3};
  return t == parts ? 0 : 1;
}
