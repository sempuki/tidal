// tidal-rules: CTAD-18
// tidal-expect: accept
// tidal-expect-edg: reject
// tidal-error-edg: static assertion failed
// tidal-note: Gap review RV8-B-8. A guide whose first parameter is a reference to const std::initializer_list<int> is an initializer-list guide, so the braced list of two chooses it in the first phase over the guide (int, int), and the type is Bag<long>, as Clang and GCC do. EDG chooses the guide (int, int) and gives Bag<char> (Q-0443).
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
  constexpr Bag(int, int) {}
  constexpr Bag(const std::initializer_list<int>&) {}
};
Bag(const std::initializer_list<int>&) -> Bag<long>;
Bag(int, int) -> Bag<char>;
Bag pair{1, 2};
static_assert(same<decltype(pair), Bag<long>>);
