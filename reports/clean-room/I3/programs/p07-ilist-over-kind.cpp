// ICS-21 before ICS-22 (Q-0133): the standard sequence to int wins.
namespace std {
template <class E> class initializer_list {
  const E* first_;
  decltype(sizeof 0) size_;
  constexpr initializer_list(const E* f, decltype(sizeof 0) n) : first_(f), size_(n) {}
public:
  constexpr initializer_list() : first_(nullptr), size_(0) {}
};
}
struct Num { Num(int) {} };
int take(std::initializer_list<Num>) { return 1; }
int take(int) { return 2; }
int main() { return take({1}) == 2 ? 0 : 1; }
