// notes 20: nested braced elements give initializer_list<T> no value.
namespace std {
template <class E> class initializer_list {
  const E* b_;
  decltype(sizeof 0) n_;
  constexpr initializer_list(const E* b, decltype(sizeof 0) n) : b_(b), n_(n) {}
public:
  constexpr initializer_list() : b_(nullptr), n_(0) {}
};
}
template <class T> int f(std::initializer_list<T>) { return 1; }
int f(std::initializer_list<long>) { return 2; }
int main() { return f({1, 2}) == 1 && f({{1}, {2}}) == 2 ? 0 : 1; }
