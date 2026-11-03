// tidal-rules: SB-5 SB-13
// tidal-expect: reject
// tidal-error-clang: (cannot bind|cannot decompose) private member
// tidal-error-gcc: cannot decompose inaccessible member
// tidal-error-edg: cannot bind to non-public member
// tidal-note: Write reports W5 and W6. E is const Range, std::tuple_size<const Range> is incomplete, so the names go to the private data members.
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
template <decltype(sizeof 0) I> struct std::tuple_element<I, Range> { using type = int; };
int width(const Range& r) {
  const auto& [lo, hi] = r;
  return hi - lo;
}
