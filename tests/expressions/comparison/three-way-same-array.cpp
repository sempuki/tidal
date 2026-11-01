// tidal-rules: CMP-24
// tidal-expect: reject
// tidal-error-clang: invalid operands to binary expression
// tidal-error-gcc: invalid operands of types .int \[2\]. and .int \[2\]. to binary .operator<=>.
// tidal-error-edg: invalid types .*int \[2\].* for built-in operator<=>
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
int a[2];
auto r = a <=> a;
