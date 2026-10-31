// tidal-rules: ILC-3
// tidal-expect: run
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
struct P { int a, b; };
int total = 0;
struct S { S(std::initializer_list<P> l) { for (const P& p : l) total += p.a * p.b; } };
int main() { S s{{1, 2}, {3, 4}}; return total == 14 ? 0 : 1; }
