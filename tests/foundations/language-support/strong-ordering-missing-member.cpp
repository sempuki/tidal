// tidal-rules: LSUP-2
// tidal-expect: reject
// tidal-error-clang: member .equal. is missing
// tidal-error-gcc: .equal. is not a member of
// tidal-error-edg: no constexpr static data member "equal"
namespace std {
class partial_ordering {
  signed char value;
  constexpr explicit partial_ordering(signed char v) : value(v) {}
public:
  static const partial_ordering less, equivalent, greater, unordered;
  friend constexpr bool operator==(partial_ordering, partial_ordering) = default;
  friend constexpr bool operator<(partial_ordering a, int) { return a.value == -1; }
};
inline constexpr partial_ordering partial_ordering::less{-1};
inline constexpr partial_ordering partial_ordering::equivalent{0};
inline constexpr partial_ordering partial_ordering::greater{1};
inline constexpr partial_ordering partial_ordering::unordered{2};
class weak_ordering {
  signed char value;
  constexpr explicit weak_ordering(signed char v) : value(v) {}
public:
  static const weak_ordering less, equivalent, greater;
  friend constexpr bool operator==(weak_ordering, weak_ordering) = default;
};
inline constexpr weak_ordering weak_ordering::less{-1};
inline constexpr weak_ordering weak_ordering::equivalent{0};
inline constexpr weak_ordering weak_ordering::greater{1};
class strong_ordering {
  signed char value;
  constexpr explicit strong_ordering(signed char v) : value(v) {}
public:
  static const strong_ordering less, equivalent, greater;
  friend constexpr bool operator==(strong_ordering, strong_ordering) = default;
  friend constexpr bool operator<(strong_ordering a, int) { return a.value < 0; }
};
inline constexpr strong_ordering strong_ordering::less{-1};
inline constexpr strong_ordering strong_ordering::equivalent{0};
inline constexpr strong_ordering strong_ordering::greater{1};
}
auto r = 1 <=> 2;
