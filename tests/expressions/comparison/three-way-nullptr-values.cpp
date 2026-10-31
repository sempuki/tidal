// tidal-rules: CMP-25
// tidal-expect: reject
// tidal-error-clang: invalid operands to binary expression .*nullptr_t
// tidal-error-gcc: ordered comparison of pointer with integer zero
// tidal-error-edg: for built-in operator<=>
// tidal-note: Gap review 2026-10-31 (third).
namespace std {
struct strong_ordering {
  signed char value;
  static const strong_ordering less, equal, equivalent, greater;
  friend constexpr bool operator==(strong_ordering, strong_ordering) = default;
};
inline constexpr strong_ordering strong_ordering::less{-1};
inline constexpr strong_ordering strong_ordering::equal{0};
inline constexpr strong_ordering strong_ordering::equivalent{0};
inline constexpr strong_ordering strong_ordering::greater{1};
struct weak_ordering {
  signed char value;
  static const weak_ordering less, equivalent, greater;
  friend constexpr bool operator==(weak_ordering, weak_ordering) = default;
};
inline constexpr weak_ordering weak_ordering::less{-1};
inline constexpr weak_ordering weak_ordering::equivalent{0};
inline constexpr weak_ordering weak_ordering::greater{1};
struct partial_ordering {
  signed char value;
  static const partial_ordering less, equivalent, greater, unordered;
  friend constexpr bool operator==(partial_ordering, partial_ordering) = default;
};
inline constexpr partial_ordering partial_ordering::less{-1};
inline constexpr partial_ordering partial_ordering::equivalent{0};
inline constexpr partial_ordering partial_ordering::greater{1};
inline constexpr partial_ordering partial_ordering::unordered{2};
}
auto r = nullptr <=> nullptr;
