// tidal-rules: LSUP-2
// tidal-expect: reject
// tidal-expect-gcc: run
// tidal-note: Q-0402. Roadmap round 7, programmer reduction ordering_declared_after_class.cpp (reports/clean-room/roadmap-7/programmer.md, section 1.5). The class declares a defaulted <=> before std::strong_ordering is declared, and the program uses it after. Clang and EDG reject at the defaulted declaration; GCC accepts.
struct P {
  int a;
  auto operator<=>(const P&) const = default;
};
namespace std {
struct strong_ordering {
  signed char v;
  static const strong_ordering less, equal, equivalent, greater;
  friend constexpr bool operator==(strong_ordering a, int) { return a.v == 0; }
  friend constexpr bool operator<(strong_ordering a, int) { return a.v < 0; }
};
inline constexpr strong_ordering strong_ordering::less{-1};
inline constexpr strong_ordering strong_ordering::equal{0};
inline constexpr strong_ordering strong_ordering::equivalent{0};
inline constexpr strong_ordering strong_ordering::greater{1};
}
int main() { return P{1} < P{2} ? 0 : 1; }
