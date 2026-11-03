// tidal-rules: SB-5 SB-4 LSUP-3
// tidal-expect: accept
// tidal-note: Write reports W5 and W6. A specialization for const Range serves a const hidden variable. decltype of a name is the tuple_element type as written, with no cv-qualifiers added.
template <class A, class B> constexpr bool same = false;
template <class A> constexpr bool same<A, A> = true;
namespace std {
template <class T> struct tuple_size;
template <decltype(sizeof 0) I, class T> struct tuple_element;
}
class Range {
  int low, high;
public:
  constexpr Range(int l, int h) : low(l), high(h) {}
  template <decltype(sizeof 0) I> constexpr int get() const { return I == 0 ? low : high; }
};
template <> struct std::tuple_size<Range> { static constexpr decltype(sizeof 0) value = 2; };
template <> struct std::tuple_size<const Range> { static constexpr decltype(sizeof 0) value = 2; };
template <decltype(sizeof 0) I> struct std::tuple_element<I, Range> { using type = int; };
template <decltype(sizeof 0) I> struct std::tuple_element<I, const Range> { using type = int; };
constexpr int width(const Range& r) {
  const auto& [lo, hi] = r;
  static_assert(same<decltype(lo), int>);
  return hi - lo;
}
static_assert(width(Range{2, 7}) == 5);
constexpr int sum() {
  const auto [lo, hi] = Range{1, 2};
  static_assert(same<decltype(hi), int>);
  return lo + hi;
}
static_assert(sum() == 3);
