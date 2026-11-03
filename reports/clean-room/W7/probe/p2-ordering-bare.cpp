// Probe: LSUP-2 shape only (static members, no operators).
namespace std {
struct strong_ordering {
  signed char value;
  static const strong_ordering less, equal, equivalent, greater;
};
inline constexpr strong_ordering strong_ordering::less{-1};
inline constexpr strong_ordering strong_ordering::equal{0};
inline constexpr strong_ordering strong_ordering::equivalent{0};
inline constexpr strong_ordering strong_ordering::greater{1};
}
struct P { int a, b; auto operator<=>(const P&) const = default; };
#ifdef USE_LESS
static_assert(P{1, 2} < P{1, 3});
#endif
#ifdef USE_SPACESHIP
constexpr auto r = P{1, 2} <=> P{1, 3};
#endif
#ifdef USE_EQ
static_assert(P{1, 2} == P{1, 2});
#endif
int main() { return 0; }
