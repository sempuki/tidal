// tidal-rules: CMP-13
// tidal-expect: reject
// tidal-flags: -m32
// tidal-error-clang: cannot be narrowed from type .long. to .unsigned long.
// tidal-error-gcc: narrowing conversion of .balance. from .long int. to .long unsigned int.
// tidal-error-edg: invalid narrowing conversion from .long. to .unsigned long.
// tidal-note: The usual arithmetic conversions give unsigned long when long and unsigned have the same size (ILP32), and long to unsigned long narrows.
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
static_assert(sizeof(long) == sizeof(unsigned));
long balance = -1;
unsigned count = 1;
auto r = balance <=> count;
