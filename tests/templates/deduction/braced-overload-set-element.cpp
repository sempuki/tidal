// tidal-rules: DED-16
// tidal-expect: accept
// tidal-note: The element act names an overload set and gives nothing. The element f gives T, and act then converts to F.
template <class A, class B> constexpr bool same = false;
template <class A> constexpr bool same<A, A> = true;
template <class T> struct Tag {};
namespace std {
template <class E> class initializer_list {
  const E* b_;
  decltype(sizeof 0) n_;
  constexpr initializer_list(const E* b, decltype(sizeof 0) n) : b_(b), n_(n) {}
public:
  constexpr initializer_list() : b_(nullptr), n_(0) {}
};
}
void act(int);
void act(long);
template <class T> Tag<T> list(std::initializer_list<T>);
using F = void (*)(int);
F f = nullptr;
static_assert(same<decltype(list({f, act})), Tag<F>>);
