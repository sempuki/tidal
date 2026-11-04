// tidal-rules: ICS-16
// tidal-expect: accept
// tidal-note: A designated list has no sequence to std::initializer_list, so ICS-22 does not apply and Point is chosen.
namespace std {
template <class E> class initializer_list {
  const E* first_;
  decltype(sizeof 0) size_;
  constexpr initializer_list(const E* f, decltype(sizeof 0) n) : first_(f), size_(n) {}
public:
  constexpr initializer_list() : first_(nullptr), size_(0) {}
};
}
template <class T, class U> constexpr bool same = false;
template <class T> constexpr bool same<T, T> = true;
struct One {};
struct Two {};
struct Point { int x; };
One take(std::initializer_list<int>);
Two take(Point);
static_assert(same<decltype(take({.x = 1})), Two>);
