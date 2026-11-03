// tidal-rules: LSUP-13
// tidal-expect: reject
// tidal-error-clang: no matching conversion for static_cast from .std::strong_ordering. to .std::partial_ordering
// tidal-error-gcc: does not convert to .std::partial_ordering
// tidal-error-edg: no suitable user-defined conversion from "std::strong_ordering" to "std::partial_ordering"
// tidal-note: The int member gives strong_ordering, the declared return type is partial_ordering, and no conversion exists.
namespace std {
class partial_ordering {
  signed char v;
public:
  constexpr explicit partial_ordering(signed char x) : v(x) {}
  static const partial_ordering less, equivalent, greater, unordered;
  friend constexpr bool operator==(partial_ordering, partial_ordering) = default;
  friend constexpr bool operator==(partial_ordering a, int) { return a.v == 0; }
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
};
inline constexpr strong_ordering strong_ordering::less{-1};
inline constexpr strong_ordering strong_ordering::equal{0};
inline constexpr strong_ordering strong_ordering::equivalent{0};
inline constexpr strong_ordering strong_ordering::greater{1};
}
struct Count {
  int n;
  std::partial_ordering operator<=>(const Count&) const = default;
};
static_assert((Count{1} <=> Count{3}) == std::partial_ordering::less);
