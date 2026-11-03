// tidal-rules: LSUP-12
// tidal-expect: reject
// tidal-expect-clang: accept
// tidal-expect-edg: accept
// tidal-error-gcc: no match for .operator==. .operand types are .std::strong_ordering. and .int
// tidal-note: Q-0334. strong_ordering has != with a literal 0 and no ==.
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
  friend constexpr bool operator!=(strong_ordering a, int) { return a.v != 0; }
};
inline constexpr strong_ordering strong_ordering::less{-1};
inline constexpr strong_ordering strong_ordering::equal{0};
inline constexpr strong_ordering strong_ordering::equivalent{0};
inline constexpr strong_ordering strong_ordering::greater{1};
}
struct Date {
  int year, month;
  auto operator<=>(const Date&) const = default;
};
static_assert((Date{2024, 1} <=> Date{2024, 2}) == std::strong_ordering::less);
