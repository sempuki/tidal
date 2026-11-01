// tidal-rules: CMP-27 CMP-17 LSUP-2
// tidal-expect: run
// tidal-note: The program gives the members of strong_ordering values of its own choice. The built-in <=> gives a copy of the member that CMP-17 names.
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
  constexpr explicit strong_ordering(signed char v) : value(v) {}
public:
  signed char value;
  static const strong_ordering less, equal, equivalent, greater;
  friend constexpr bool operator==(strong_ordering, strong_ordering) = default;
  friend constexpr bool operator<(strong_ordering a, int) { return a.value < 0; }
};
inline constexpr strong_ordering strong_ordering::less{-7};
inline constexpr strong_ordering strong_ordering::equal{5};
inline constexpr strong_ordering strong_ordering::equivalent{6};
inline constexpr strong_ordering strong_ordering::greater{9};
}
static_assert((1 <=> 2).value == -7);
static_assert((2 <=> 2).value == 5);
static_assert((3 <=> 2).value == 9);
int main(int argc, char**) {
  int volatile a = argc, b = 1;
  int lo = (a - 1 <=> b).value, eq = (a <=> b).value, hi = (a + 1 <=> b).value;
  return lo == -7 && eq == 5 && hi == 9 ? 0 : 1;
}
