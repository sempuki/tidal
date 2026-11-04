// tidal-rules: ICS-22
// tidal-expect: accept
// tidal-note: Both sequences are the identity and both are standard, so kind, rank, and the tie-breakers give no answer. Only ICS-22 decides.
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
One take(std::initializer_list<int>);
Two take(int);
static_assert(same<decltype(take({1})), One>);
