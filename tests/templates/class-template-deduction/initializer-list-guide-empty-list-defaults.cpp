// tidal-rules: CTAD-18
// tidal-expect: accept
// tidal-expect-gcc: reject
// tidal-error-gcc: static assertion failed
// tidal-expect-edg: reject
// tidal-error-edg: static assertion failed
// tidal-note: Gap review RV8-B-8. The user guide Bag(int = 0) takes no arguments, as a guide with no parameters does, so an empty list skips the initializer-list phase and gives Bag<char>, as Clang does. GCC and EDG choose the initializer-list guide and give Bag<long> (Q-0438).
namespace std {
template <class E> class initializer_list {
  const E* b_;
  decltype(sizeof 0) n_;
  constexpr initializer_list(const E* b, decltype(sizeof 0) n) : b_(b), n_(n) {}
public:
  constexpr initializer_list() : b_(nullptr), n_(0) {}
};
}
template <class A, class B> constexpr bool same = false;
template <class A> constexpr bool same<A, A> = true;
template <class T> struct Bag {
  constexpr Bag(int = 0) {}
  constexpr Bag(std::initializer_list<int>) {}
};
Bag(std::initializer_list<int>) -> Bag<long>;
Bag(int = 0) -> Bag<char>;
Bag empty{};
static_assert(same<decltype(empty), Bag<char>>);
