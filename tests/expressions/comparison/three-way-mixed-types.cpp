// tidal-rules: CMP-13 CMP-17
// tidal-expect: accept
// tidal-note: A constant whose value fits the converted type is not narrowing.
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
static_assert((1 <=> 2u) == std::strong_ordering::less);
static_assert((7u <=> 7) == std::strong_ordering::equal);
static_assert(((unsigned char)200 <=> -1) == std::strong_ordering::greater);
static_assert((1 <=> 2LL) == std::strong_ordering::less);
static_assert((2.5f <=> 2.5) == std::partial_ordering::equivalent);
