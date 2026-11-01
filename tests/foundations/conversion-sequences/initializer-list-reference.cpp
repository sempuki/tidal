// tidal-rules: ICS-22
// tidal-expect: accept
// tidal-note: A reference to std::initializer_list reaches the list through a temporary (ICS-20). ICS-22 still prefers it over a better rank.
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
One take(int);
Two take(const std::initializer_list<long>&);
static_assert(same<decltype(take({1})), Two>);
One wide(long);
Two wide(const std::initializer_list<int>&);
static_assert(same<decltype(wide({1})), Two>);
One move(std::initializer_list<int>&&);
Two move(int);
static_assert(same<decltype(move({1})), One>);
