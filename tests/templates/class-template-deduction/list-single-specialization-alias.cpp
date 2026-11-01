// tidal-rules: CTAD-18
// tidal-expect: accept
// tidal-expect-edg: reject
// tidal-expect-clang-18: reject
// tidal-error-edg: static assertion failed
// tidal-error-clang-18: requires template arguments; argument deduction only allowed for class templates
// tidal-note: Through the alias, one element of type List<int> skips the initializer-list phase, and the copy candidate gives List<int>. EDG uses the initializer-list guide and gives List<List<int>> (Q-0279). Clang 18 has no deduction through an alias template (Q-0240).
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
template <class T> struct List { constexpr List(std::initializer_list<T>) {} };
template <class T> using Alias = List<T>;
constexpr List<int> one{1, 2};
Alias copy{one};
Alias two{one, one};
static_assert(same<decltype(copy), List<int>> && same<decltype(two), List<List<int>>>);
