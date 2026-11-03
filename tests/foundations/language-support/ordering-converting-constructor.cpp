// tidal-rules: LSUP-13
// tidal-expect: accept
// tidal-note: Write report W2. A converting constructor on the weaker class serves in place of a conversion function.
namespace std {
class strong_ordering {
  signed char v;
public:
  constexpr explicit strong_ordering(signed char x) : v(x) {}
  constexpr signed char raw() const { return v; }
  static const strong_ordering less, equal, equivalent, greater;
  friend constexpr bool operator==(strong_ordering, strong_ordering) = default;
  friend constexpr bool operator==(strong_ordering a, int) { return a.v == 0; }
};
inline constexpr strong_ordering strong_ordering::less{-1};
inline constexpr strong_ordering strong_ordering::equal{0};
inline constexpr strong_ordering strong_ordering::equivalent{0};
inline constexpr strong_ordering strong_ordering::greater{1};
class weak_ordering {
  signed char v;
public:
  constexpr explicit weak_ordering(signed char x) : v(x) {}
  constexpr weak_ordering(strong_ordering s) : v(s.raw()) {}
  constexpr signed char raw() const { return v; }
  static const weak_ordering less, equivalent, greater;
  friend constexpr bool operator==(weak_ordering, weak_ordering) = default;
  friend constexpr bool operator==(weak_ordering a, int) { return a.v == 0; }
};
inline constexpr weak_ordering weak_ordering::less{-1};
inline constexpr weak_ordering weak_ordering::equivalent{0};
inline constexpr weak_ordering weak_ordering::greater{1};
class partial_ordering {
  signed char v;
public:
  constexpr explicit partial_ordering(signed char x) : v(x) {}
  constexpr partial_ordering(strong_ordering s) : v(s.raw()) {}
  constexpr partial_ordering(weak_ordering w) : v(w.raw()) {}
  static const partial_ordering less, equivalent, greater, unordered;
  friend constexpr bool operator==(partial_ordering, partial_ordering) = default;
  friend constexpr bool operator==(partial_ordering a, int) { return a.v == 0; }
};
inline constexpr partial_ordering partial_ordering::less{-1};
inline constexpr partial_ordering partial_ordering::equivalent{0};
inline constexpr partial_ordering partial_ordering::greater{1};
inline constexpr partial_ordering partial_ordering::unordered{2};
}
struct Reading {
  int sensor;
  double value;
  auto operator<=>(const Reading&) const = default;
};
static_assert((Reading{1, 2.0} <=> Reading{1, 3.0}) == std::partial_ordering::less);
static_assert((Reading{2, 0.0} <=> Reading{1, 9.0}) == std::partial_ordering::greater);
