namespace std {
template <class E> class initializer_list {
  const E* b_;
  decltype(sizeof 0) n_;
  constexpr initializer_list(const E* b, decltype(sizeof 0) n) : b_(b), n_(n) {}
public:
  constexpr initializer_list() : b_(nullptr), n_(0) {}
};
}
struct Bag { Bag(std::initializer_list<int>) {} };
int main() { Bag a{1}; Bag b{a}; return 0; }
