// tidal-rules: CTAD-18
// tidal-expect: accept
// tidal-note: One element of type List<int>, or of a class derived from it, does not make List<List<int>>: the initializer-list guide takes no part, and the copy candidate gives List<int>. Two elements use the initializer-list guide.
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
constexpr List one{1, 2};
List copy{one};
List copy_list = {one};
List nested{one, one};
static_assert(same<decltype(one), const List<int>> && same<decltype(copy), List<int>>);
static_assert(same<decltype(copy_list), List<int>> && same<decltype(nested), List<List<int>>>);
struct More : List<int> { constexpr More() : List<int>{1} {} };
constexpr More more;
List from_derived{more};
static_assert(same<decltype(from_derived), List<int>>);
List single{1L};
static_assert(same<decltype(single), List<long>>);
