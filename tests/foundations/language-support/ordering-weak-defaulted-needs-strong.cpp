// tidal-rules: LSUP-2
// tidal-expect: reject
// tidal-expect-gcc-14: run-fail
// tidal-error-clang: std::strong_ordering. was not found
// tidal-note: Gap review 2026-11-05 (reports/review/2026-11-05-gap-review-phase-9-declarations-statements-foundations.md). A defaulted <=> that returns std::weak_ordering gives, when every part compares equal, strong_ordering::equal converted to the return type. So the program must declare strong_ordering too. This class has no parts, and only weak_ordering is declared. Clang, GCC 16, and EDG reject. GCC 14 accepts it, and the program stops at run time.
namespace std {
struct weak_ordering {
  signed char v;
  static const weak_ordering less, equivalent, greater;
  friend constexpr bool operator==(weak_ordering a, int) { return a.v == 0; }
};
inline constexpr weak_ordering weak_ordering::less{-1};
inline constexpr weak_ordering weak_ordering::equivalent{0};
inline constexpr weak_ordering weak_ordering::greater{1};
}
struct P { std::weak_ordering operator<=>(const P&) const = default; };
int main() { return (P{} <=> P{}) == 0 ? 0 : 1; }
