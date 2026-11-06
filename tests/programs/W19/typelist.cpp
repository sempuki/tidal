// tidal-rules: PACK-19 TSP-14 PACK-12 LAM-15 DEP-24
// tidal-expect: run
// tidal-note: Write suite W19, program typelist.cpp (reports/clean-room/W19/report.md). The rules are the ones the report's notes made before writing give for this program.
// tidal-note: A program test is evidence from use and decides no rule alone (decision 0054).
// typelist.cpp: a compile-time type list with length, index-of, and map.
namespace std { using size_t = decltype(sizeof 0); }

template <class... Ts> struct List {};           // TPL-30, PACK-P1

template <class T> struct Type { using type = T; };  // a type as a value
template <class B> using Unwrap = typename B::type;  // DEP-22

// same<A, B>: a variable template with a partial specialization (TSP-10).
template <class A, class B> constexpr bool same = false;
template <class A> constexpr bool same<A, A> = true;

// length, by pattern matching on List<Ts...> (TSP-14, PACK-19).
template <class L> struct Length;
template <class... Ts> struct Length<List<Ts...>> {
  static constexpr std::size_t value = sizeof...(Ts);
};

// index_of: a constexpr function over the pack (CX-3, PRIM-27).
template <class T, class... Ts>
constexpr std::size_t index_of(List<Ts...>) {
  constexpr bool hits[] = {same<T, Ts>..., false};
  for (std::size_t i = 0; i < sizeof...(Ts); ++i)
    if (hits[i]) return i;
  return sizeof...(Ts);  // not found: one past the end, like length
}

// map f over the list. f has a template head (LAM-15) and returns Type<U>.
// f has a dependent type, so the call needs the keyword template (DEP-24).
template <class... Ts, class F>
constexpr auto map(List<Ts...>, F f) {
  // Unwrap<...> in place of typename decltype(...)::type, which tidal-fe
  // rejects (W19 reductions/typename-decltype-nested.cpp).
  return List<Unwrap<decltype(f.template operator()<Ts>())>...>{};
}

using L = List<int, char, double>;
static_assert(Length<L>::value == 3);
static_assert(Length<List<>>::value == 0);
static_assert(index_of<char>(L{}) == 1);
static_assert(index_of<double>(L{}) == 2);
static_assert(index_of<long>(L{}) == 3);

constexpr auto ptrs = map(L{}, []<class T>() { return Type<T*>{}; });
static_assert(same<decltype(ptrs), const List<int*, char*, double*>>);
static_assert(same<decltype(map(List<>{}, []<class T>() { return Type<T>{}; })), List<>>);

int main() {
  // At run time: call the mapped lambda and the index function.
  auto sizes = []<class... Ts>(List<Ts...>) { return (sizeof(Ts) + ... + 0); };
  if (sizes(L{}) != sizeof(int) + sizeof(char) + sizeof(double)) return 1;
  if (index_of<int>(L{}) != 0) return 2;
  return 0;
}
