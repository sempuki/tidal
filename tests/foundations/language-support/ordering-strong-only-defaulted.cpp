// tidal-rules: LSUP-2
// tidal-expect: reject
// tidal-expect-gcc: accept
// tidal-expect-clang: accept
// tidal-error-edg: no valid std::weak_ordering type found
// tidal-note: Q-0386. The program declares std::strong_ordering and neither of the other two ordering classes. The defaulted <=> compares two int members, so its result type is std::strong_ordering. The class has == and != against 0 for the defaulted operator (LSUP-12).
namespace std {
struct strong_ordering {
  signed char v;
  static const strong_ordering less, equal, equivalent, greater;
  friend constexpr bool operator==(strong_ordering s, int) { return s.v == 0; }
  friend constexpr bool operator!=(strong_ordering s, int) { return s.v != 0; }
};
inline constexpr strong_ordering strong_ordering::less{-1};
inline constexpr strong_ordering strong_ordering::equal{0};
inline constexpr strong_ordering strong_ordering::equivalent{0};
inline constexpr strong_ordering strong_ordering::greater{1};
}
struct Version {
  int major, minor;
  auto operator<=>(const Version&) const = default;
};
int main() {
  std::strong_ordering r = Version{1, 2} <=> Version{1, 3};
  return r.v == -1 ? 0 : 1;
}
