// tidal-rules: CMP-27
// tidal-expect: accept
// tidal-note: The program gives the members of partial_ordering and strong_ordering values of its own choice. The built-in <=> on floating-point operands gives a copy of a partial_ordering member, and on pointers a copy of a strong_ordering member.
namespace std {
class partial_ordering {
  constexpr explicit partial_ordering(signed char v) : value(v) {}
public:
  signed char value;
  static const partial_ordering less, equivalent, greater, unordered;
  friend constexpr bool operator==(partial_ordering, partial_ordering) = default;
};
inline constexpr partial_ordering partial_ordering::less{-3};
inline constexpr partial_ordering partial_ordering::equivalent{4};
inline constexpr partial_ordering partial_ordering::greater{8};
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
};
inline constexpr strong_ordering strong_ordering::less{-5};
inline constexpr strong_ordering strong_ordering::equal{3};
inline constexpr strong_ordering strong_ordering::equivalent{3};
inline constexpr strong_ordering strong_ordering::greater{7};
}
static_assert((1.0 <=> 2.0).value == -3);
static_assert((2.0f <=> 2.0f).value == 4);
static_assert((3.0 <=> 2.0).value == 8);
constexpr int cells[3] = {};
static_assert((&cells[0] <=> &cells[2]).value == -5);
static_assert((&cells[1] <=> &cells[1]).value == 3);
static_assert((&cells[2] <=> &cells[0]).value == 7);
