// tidal-rules: ICS-16
// tidal-expect: accept
// tidal-note: The worst element decides: a promotion for int, a conversion for long, a user-defined conversion for Num.
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
struct Num { Num(int); };
short small = 1;
One take(std::initializer_list<int>);
Two take(std::initializer_list<long>);
static_assert(same<decltype(take({small, 1})), One>);
One put(std::initializer_list<Num>);
Two put(std::initializer_list<long>);
static_assert(same<decltype(put({1, 2})), Two>);
One none(std::initializer_list<int>);
Two none(...);
static_assert(same<decltype(none({})), One>);
