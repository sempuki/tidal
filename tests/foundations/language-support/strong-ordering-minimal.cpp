// tidal-rules: LSUP-2
// tidal-expect: accept
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
  static const strong_ordering less, equal, equivalent, greater;
  friend constexpr bool operator==(strong_ordering, strong_ordering) = default;
  friend constexpr bool operator<(strong_ordering a, int) { return a.value < 0; }
};
inline constexpr strong_ordering strong_ordering::less{-1};
inline constexpr strong_ordering strong_ordering::equal{0};
inline constexpr strong_ordering strong_ordering::equivalent{0};
inline constexpr strong_ordering strong_ordering::greater{1};
}
static_assert((1 <=> 2) < 0);
static_assert((2 <=> 2) == std::strong_ordering::equal);
static_assert((1.0 <=> 2.0) < 0);
