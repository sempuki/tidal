// tidal-rules: PACK-13 PACK-14 SB-5 PRIM-27 TSP-15 DED-35
// tidal-expect: run
// tidal-note: Roadmap round 10, programmer sketch tuple_second_try.cpp (reports/clean-room/roadmap-10/programmer.md, section 1.4). The first try differs only in a fold without parentheses, on which Clang warns, so the second try stands for the first try (decision 0061). The rules are the ones the report names for this program and its reductions, and the rules of the main constructs it uses.
// tidal-note: A program test is evidence from use and decides no rule alone (decision 0054).
// A Tuple<Ts...> built from indexed leaves by pack expansion of bases, get<I>
// for lvalues and rvalues, the tuple protocol for structured bindings
// (tuple_size and tuple_element in namespace std, member get), an index
// sequence, apply that expands get<I>(t)... into a call, make_tuple with
// decay of string literals and arrays to pointers, tuple_cat of two
// tuples, a lexicographic == and <=> written with folds, and a for_each
// over the elements. Every check also runs in constant evaluation.
namespace std {
using size_t = decltype(sizeof 0);
template <class T> struct tuple_size;
template <size_t I, class T> struct tuple_element;
}

template <class T> struct remove_ref { using type = T; };
template <class T> struct remove_ref<T&> { using type = T; };
template <class T> struct remove_ref<T&&> { using type = T; };
template <class T> using remove_ref_t = typename remove_ref<T>::type;
template <class T> constexpr T&& forward(remove_ref_t<T>& t) noexcept { return static_cast<T&&>(t); }
template <class T> constexpr remove_ref_t<T>&& move(T&& t) noexcept { return static_cast<remove_ref_t<T>&&>(t); }

template <class T> struct decay_of { using type = T; };
template <class T> struct decay_of<const T> { using type = T; };
template <class T, std::size_t N> struct decay_of<T[N]> { using type = T*; };
template <class T, std::size_t N> struct decay_of<const T[N]> { using type = const T*; };
template <class T> using decay_t = typename decay_of<remove_ref_t<T>>::type;

template <std::size_t... I> struct index_seq {};
template <std::size_t N, std::size_t... I> struct make_seq : make_seq<N - 1, N - 1, I...> {};
template <std::size_t... I> struct make_seq<0, I...> { using type = index_seq<I...>; };
template <std::size_t N> using make_index_seq = typename make_seq<N>::type;

template <std::size_t I, class T> struct Leaf {
  T value;
  constexpr Leaf() = default;
  template <class U> constexpr Leaf(U&& u) : value(forward<U>(u)) {}
};

template <class Seq, class... Ts> struct TupleImpl;
template <std::size_t... I, class... Ts>
struct TupleImpl<index_seq<I...>, Ts...> : Leaf<I, Ts>... {
  constexpr TupleImpl() = default;
  template <class... Us> constexpr explicit TupleImpl(int, Us&&... us) : Leaf<I, Ts>(forward<Us>(us))... {}
};

template <class... Ts> struct Tuple : TupleImpl<make_index_seq<sizeof...(Ts)>, Ts...> {
  using Base = TupleImpl<make_index_seq<sizeof...(Ts)>, Ts...>;
  constexpr Tuple() = default;
  template <class... Us> requires (sizeof...(Us) == sizeof...(Ts) && sizeof...(Ts) > 0)
  constexpr Tuple(Us&&... us) : Base(0, forward<Us>(us)...) {}

  template <std::size_t I> constexpr auto& get() & { return leaf<I>(*this).value; }
  template <std::size_t I> constexpr const auto& get() const& { return leaf<I>(*this).value; }
  template <std::size_t I> constexpr auto&& get() && { return move(leaf<I>(*this).value); }
private:
  template <std::size_t I, class T> static constexpr Leaf<I, T>& leaf(Leaf<I, T>& l) { return l; }
  template <std::size_t I, class T> static constexpr const Leaf<I, T>& leaf(const Leaf<I, T>& l) { return l; }
};
template <class... Us> Tuple(Us...) -> Tuple<Us...>;

template <class... Ts> struct std::tuple_size<Tuple<Ts...>> { static constexpr std::size_t value = sizeof...(Ts); };
template <std::size_t I, class T, class... Ts> struct std::tuple_element<I, Tuple<T, Ts...>> : std::tuple_element<I - 1, Tuple<Ts...>> {};
template <class T, class... Ts> struct std::tuple_element<0, Tuple<T, Ts...>> { using type = T; };

template <std::size_t I, class Tup> constexpr decltype(auto) get(Tup&& t) {
  return forward<Tup>(t).template get<I>();
}

template <class... Us> constexpr Tuple<decay_t<Us>...> make_tuple(Us&&... us) {
  return Tuple<decay_t<Us>...>(forward<Us>(us)...);
}

template <class F, class Tup, std::size_t... I>
constexpr decltype(auto) apply_impl(F&& f, Tup&& t, index_seq<I...>) {
  return forward<F>(f)(get<I>(forward<Tup>(t))...);
}
template <class F, class... Ts> constexpr decltype(auto) apply(F&& f, Tuple<Ts...>& t) {
  return apply_impl(forward<F>(f), t, make_index_seq<sizeof...(Ts)>{});
}

template <class... As, class... Bs, std::size_t... I, std::size_t... J>
constexpr Tuple<As..., Bs...> cat_impl(const Tuple<As...>& a, const Tuple<Bs...>& b, index_seq<I...>, index_seq<J...>) {
  return Tuple<As..., Bs...>(get<I>(a)..., get<J>(b)...);
}
template <class... As, class... Bs>
constexpr Tuple<As..., Bs...> tuple_cat(const Tuple<As...>& a, const Tuple<Bs...>& b) {
  return cat_impl(a, b, make_index_seq<sizeof...(As)>{}, make_index_seq<sizeof...(Bs)>{});
}

template <class... Ts, std::size_t... I>
constexpr bool eq_impl(const Tuple<Ts...>& a, const Tuple<Ts...>& b, index_seq<I...>) {
  return ((get<I>(a) == get<I>(b)) && ...);
}
template <class... Ts> constexpr bool operator==(const Tuple<Ts...>& a, const Tuple<Ts...>& b) {
  return eq_impl(a, b, make_index_seq<sizeof...(Ts)>{});
}
template <class... Ts, std::size_t... I>
constexpr int cmp_impl(const Tuple<Ts...>& a, const Tuple<Ts...>& b, index_seq<I...>) {
  int r = 0;
  (void)(((r = get<I>(a) < get<I>(b) ? -1 : get<I>(b) < get<I>(a) ? 1 : 0, r != 0)) || ...);
  return r;
}
template <class... Ts> constexpr int compare(const Tuple<Ts...>& a, const Tuple<Ts...>& b) {
  return cmp_impl(a, b, make_index_seq<sizeof...(Ts)>{});
}

template <class F, class... Ts, std::size_t... I>
constexpr void each_impl(Tuple<Ts...>& t, F& f, index_seq<I...>) { (f(get<I>(t)), ...); }
template <class F, class... Ts> constexpr void for_each(Tuple<Ts...>& t, F f) {
  each_impl(t, f, make_index_seq<sizeof...(Ts)>{});
}

constexpr int length(const char* s) { int n = 0; while (s[n]) ++n; return n; }

struct Moved {
  int v; bool from = false;
  constexpr Moved(int v) : v(v) {}
  constexpr Moved(Moved&& o) : v(o.v) { o.from = true; }
};

constexpr int check() {
  int fails = 0;
  Tuple<int, double, const char*> t(1, 2.5, "three");
  auto& [a, b, c] = t;
  a += 10;
  if (get<0>(t) != 11 || b != 2.5 || length(c) != 5) ++fails;

  char buf[] = "ab";
  auto m = make_tuple(1, "lit", buf);
  static_assert(sizeof(get<1>(m)) == sizeof(const char*));
  if (get<2>(m) != buf) ++fails;

  int sum = apply([](int x, double y, const char* s) { return x + static_cast<int>(y) + length(s); }, t);
  if (sum != 11 + 2 + 5) ++fails;

  auto both = tuple_cat(Tuple<int, char>(1, 'x'), Tuple<long>(9L));
  static_assert(std::tuple_size<decltype(both)>::value == 3);
  auto [p, q, r] = both;
  if (p != 1 || q != 'x' || r != 9L) ++fails;

  if (!(Tuple(1, 2) == Tuple(1, 2)) || Tuple(1, 2) == Tuple(1, 3)) ++fails;
  if (compare(Tuple(1, 5), Tuple(2, 0)) != -1 || compare(Tuple(2, 0), Tuple(1, 5)) != 1) ++fails;

  int total = 0;
  Tuple<int, int, int> three(1, 2, 3);
  for_each(three, [&total](int& x) { x *= 2; total += x; });
  if (total != 12 || get<2>(three) != 6) ++fails;

  Tuple<Moved, int> mt(Moved(4), 5);
  Moved taken = get<0>(move(mt));
  if (taken.v != 4 || !get<0>(mt).from) ++fails;

  Tuple<> empty;
  (void)empty;
  static_assert(std::tuple_size<Tuple<>>::value == 0);
  return fails;
}

static_assert(check() == 0);
int main() { return check(); }
