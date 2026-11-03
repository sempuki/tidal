// tidal-rules: ICS-22 ICS-21
// tidal-expect: accept
// tidal-expect-gcc: reject
// tidal-expect-edg: reject
// tidal-error-gcc: static assertion failed
// tidal-error-edg: static assertion failed
// tidal-note: Q-0133. The sequence to the initializer list is user-defined and the one to int is standard. ICS-21 compares the kinds first, so take(int) wins. GCC and EDG apply ICS-22 first and choose the initializer list.
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
One take(std::initializer_list<Num>);
Two take(int);
static_assert(same<decltype(take({1})), Two>);
