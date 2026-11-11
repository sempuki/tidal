// lib: algorithms over iterators as function objects. lib::swap is a
// customization point: it calls a swap that argument-dependent lookup
// finds, and moves the values otherwise. lib::invoke calls a callable or
// reads a member through a pointer to member, so each algorithm takes a
// projection. sort (insertion), lower_bound, reverse, rotate, unique,
// and merge, with concepts for iterators, and constant tests.
namespace std {
using size_t = decltype(sizeof 0);
}

template <class T> struct remove_ref { using type = T; };
template <class T> struct remove_ref<T&> { using type = T; };
template <class T> struct remove_ref<T&&> { using type = T; };
template <class T> using remove_ref_t = typename remove_ref<T>::type;
template <class T> constexpr T&& forward(remove_ref_t<T>& t) noexcept { return static_cast<T&&>(t); }
template <class T> constexpr remove_ref_t<T>&& move(T&& t) noexcept { return static_cast<remove_ref_t<T>&&>(t); }
template <class T> T&& declval() noexcept;

template <class T> inline constexpr bool is_member_pointer = false;
template <class M, class C> inline constexpr bool is_member_pointer<M C::*> = true;

namespace lib {

template <class F, class A>
constexpr decltype(auto) invoke(F&& f, A&& a) {
  if constexpr (is_member_pointer<remove_ref_t<F>>) {
    if constexpr (requires { (forward<A>(a).*f)(); }) return (forward<A>(a).*f)();
    else return (forward<A>(a).*f);
  } else {
    return forward<F>(f)(forward<A>(a));
  }
}

struct identity {
  template <class T> constexpr T&& operator()(T&& t) const { return forward<T>(t); }
};
struct less {
  template <class A, class B> constexpr bool operator()(const A& a, const B& b) const { return a < b; }
};

namespace detail {
template <class T> void swap(T&, T&) = delete;

template <class T>
concept adl_swappable = requires(T& a, T& b) { swap(a, b); };

struct swap_fn {
  template <class T>
    requires adl_swappable<T>
  constexpr void operator()(T& a, T& b) const { swap(a, b); }

  template <class T>
  constexpr void operator()(T& a, T& b) const {
    T t = move(a);
    a = move(b);
    b = move(t);
  }
};
}  // namespace detail

inline constexpr detail::swap_fn swap{};

template <class T>
concept boolean = requires(T t) { static_cast<bool>(t); };

template <class I>
concept input_iterator = requires(I i) {
  *i;
  ++i;
  { i == i } -> boolean;
};
template <class I>
concept bidirectional_iterator = input_iterator<I> && requires(I i) { --i; };
template <class I>
concept random_access_iterator = bidirectional_iterator<I> && requires(I i, decltype(i - i) n) {
  i + n;
  i[n];
  { i < i } -> boolean;
};

struct sort_fn {
  template <random_access_iterator I, class Cmp = less, class Proj = identity>
  constexpr void operator()(I first, I last, Cmp cmp = {}, Proj proj = {}) const {
    if (first == last) return;
    for (I i = first + 1; i != last; ++i)
      for (I j = i; j != first && invoke(cmp, invoke(proj, *j), invoke(proj, *(j - 1))); --j)
        lib::swap(*j, *(j - 1));
  }
};
inline constexpr sort_fn sort{};

struct lower_bound_fn {
  template <random_access_iterator I, class V, class Cmp = less, class Proj = identity>
  constexpr I operator()(I first, I last, const V& v, Cmp cmp = {}, Proj proj = {}) const {
    auto n = last - first;
    while (n > 0) {
      auto half = n / 2;
      I mid = first + half;
      if (cmp(invoke(proj, *mid), v)) {
        first = mid + 1;
        n -= half + 1;
      } else {
        n = half;
      }
    }
    return first;
  }
};
inline constexpr lower_bound_fn lower_bound{};

struct reverse_fn {
  template <bidirectional_iterator I>
  constexpr void operator()(I first, I last) const {
    while (first != last && first != --last) {
      lib::swap(*first, *last);
      ++first;
    }
  }
};
inline constexpr reverse_fn reverse{};

struct rotate_fn {
  // Makes mid the first element; returns where the old first went.
  template <bidirectional_iterator I>
  constexpr I operator()(I first, I mid, I last) const {
    reverse(first, mid);
    reverse(mid, last);
    reverse(first, last);
    I r = first;
    for (I m = mid; m != last; ++m) ++r;
    return r;
  }
};
inline constexpr rotate_fn rotate{};

struct unique_fn {
  template <input_iterator I, class Proj = identity>
  constexpr I operator()(I first, I last, Proj proj = {}) const {
    if (first == last) return last;
    I out = first;
    for (I i = first; ++i != last;)
      if (!(invoke(proj, *i) == invoke(proj, *out))) *++out = move(*i);
    return ++out;
  }
};
inline constexpr unique_fn unique{};

struct merge_fn {
  template <input_iterator I1, input_iterator I2, class O, class Cmp = less>
  constexpr O operator()(I1 f1, I1 l1, I2 f2, I2 l2, O out, Cmp cmp = {}) const {
    while (f1 != l1 && f2 != l2) {
      if (cmp(*f2, *f1)) *out++ = *f2++;
      else *out++ = *f1++;
    }
    while (f1 != l1) *out++ = *f1++;
    while (f2 != l2) *out++ = *f2++;
    return out;
  }
};
inline constexpr merge_fn merge{};

}  // namespace lib

// A type with its own swap, which counts its calls.
namespace cards {
struct Card {
  int rank;
  char suit;
  int* swaps;
  constexpr int score() const { return rank * 4 + (suit == 'S'); }
};
constexpr void swap(Card& a, Card& b) {
  Card t = a;
  a = b;
  b = t;
  ++*a.swaps;
}
}  // namespace cards

// A singly linked forward range over an array, for unique.
template <class T> struct Node {
  T value;
  Node* next;
};
template <class T> struct NodeIter {
  Node<T>* n;
  constexpr T& operator*() const { return n->value; }
  constexpr NodeIter& operator++() { n = n->next; return *this; }
  constexpr bool operator==(const NodeIter&) const = default;
};

static_assert(lib::random_access_iterator<int*>);
static_assert(!lib::random_access_iterator<NodeIter<int>> && lib::input_iterator<NodeIter<int>>);
static_assert(lib::detail::adl_swappable<cards::Card> && !lib::detail::adl_swappable<int>);

constexpr bool test_sort() {
  int a[] = {5, 2, 9, 1, 5, 6};
  lib::sort(a, a + 6);
  if (a[0] != 1 || a[5] != 9) return false;
  lib::sort(a, a + 6, [](int x, int y) { return x > y; });
  return a[0] == 9 && a[5] == 1;
}
static_assert(test_sort());

constexpr bool test_cards() {
  int swaps = 0;
  cards::Card h[] = {{12, 'H', &swaps}, {3, 'S', &swaps}, {7, 'D', &swaps}, {3, 'H', &swaps}};
  lib::sort(h, h + 4, lib::less{}, &cards::Card::rank);
  if (h[0].rank != 3 || h[3].rank != 12 || swaps == 0) return false;
  int before = swaps;
  lib::sort(h, h + 4, lib::less{}, &cards::Card::score);
  if (h[0].suit != 'H' || h[1].suit != 'S') return false;
  auto it = lib::lower_bound(h, h + 4, 7, lib::less{}, &cards::Card::rank);
  return it == h + 2 && swaps > before;
}
static_assert(test_cards());

constexpr bool test_rotate_unique() {
  int a[] = {1, 2, 3, 4, 5};
  int* r = lib::rotate(a, a + 2, a + 5);
  if (a[0] != 3 || a[4] != 2 || r != a + 3) return false;
  Node<int> n[] = {{1, nullptr}, {1, nullptr}, {2, nullptr}, {2, nullptr}, {3, nullptr}};
  for (int i = 0; i < 4; ++i) n[i].next = &n[i + 1];
  NodeIter<int> e = lib::unique(NodeIter<int>{n}, NodeIter<int>{nullptr});
  return e.n == &n[3] && n[0].value == 1 && n[1].value == 2 && n[2].value == 3;
}
static_assert(test_rotate_unique());

int main() {
  int x[] = {1, 4, 7, 10};
  int y[] = {2, 3, 8};
  int out[7] = {};
  int* end = lib::merge(x, x + 4, y, y + 3, out);
  if (end != out + 7) return 1;
  for (int i = 0; i + 1 < 7; ++i)
    if (out[i] > out[i + 1]) return 2;
  int big[100];
  for (int i = 0; i < 100; ++i) big[i] = (i * 37) % 100;
  lib::sort(big, big + 100);
  for (int i = 0; i < 100; ++i)
    if (big[i] != i) return 3;
  int* p = lib::lower_bound(big, big + 100, 42);
  if (p - big != 42) return 4;
  lib::reverse(big, big + 100);
  if (big[0] != 99) return 5;
  int a = 1, b = 2;
  lib::swap(a, b);
  if (a != 2 || b != 1) return 6;
  return 0;
}
