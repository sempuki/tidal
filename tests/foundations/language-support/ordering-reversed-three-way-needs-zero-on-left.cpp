// tidal-rules: LSUP-12
// tidal-expect: reject
// tidal-error-clang: invalid operands to binary expression ..int. and .std::strong_ordering
// tidal-error-gcc: no match for .operator<=>. .operand types are .int. and .std::strong_ordering
// tidal-error-edg: no operator "<=>" matches
// tidal-note: The reversed candidate for 5 <=> m is 0 <=> (m <=> 5), and strong_ordering has no <=> with 0 on the left.
namespace std {
class partial_ordering {
  signed char v;
public:
  constexpr explicit partial_ordering(signed char x) : v(x) {}
  static const partial_ordering less, equivalent, greater, unordered;
  friend constexpr bool operator==(partial_ordering, partial_ordering) = default;
};
inline constexpr partial_ordering partial_ordering::less{-1};
inline constexpr partial_ordering partial_ordering::equivalent{0};
inline constexpr partial_ordering partial_ordering::greater{1};
inline constexpr partial_ordering partial_ordering::unordered{2};
class weak_ordering {
  signed char v;
public:
  constexpr explicit weak_ordering(signed char x) : v(x) {}
  static const weak_ordering less, equivalent, greater;
  friend constexpr bool operator==(weak_ordering, weak_ordering) = default;
};
inline constexpr weak_ordering weak_ordering::less{-1};
inline constexpr weak_ordering weak_ordering::equivalent{0};
inline constexpr weak_ordering weak_ordering::greater{1};
class strong_ordering {
  signed char v;
public:
  constexpr explicit strong_ordering(signed char x) : v(x) {}
  static const strong_ordering less, equal, equivalent, greater;
  friend constexpr bool operator==(strong_ordering, strong_ordering) = default;
  friend constexpr bool operator==(strong_ordering a, int) { return a.v == 0; }
  friend constexpr bool operator<(strong_ordering a, int) { return a.v < 0; }
};
inline constexpr strong_ordering strong_ordering::less{-1};
inline constexpr strong_ordering strong_ordering::equal{0};
inline constexpr strong_ordering strong_ordering::equivalent{0};
inline constexpr strong_ordering strong_ordering::greater{1};
}
struct Meter {
  int value;
  friend constexpr std::strong_ordering operator<=>(Meter m, int i) { return m.value <=> i; }
};
static_assert(Meter{3} < 5);
static_assert((5 <=> Meter{3}) == std::strong_ordering::greater);
