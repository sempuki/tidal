// tidal-rules: LSUP-2
// tidal-expect: run
// tidal-note: Q-0386. The form of ordering-strong-only-builtin.cpp whose <=> is evaluated at run time. The use needs only the class it yields. Every compiler accepts the program.
namespace std {
struct strong_ordering {
  signed char v;
  static const strong_ordering less, equal, equivalent, greater;
};
inline constexpr strong_ordering strong_ordering::less{-1};
inline constexpr strong_ordering strong_ordering::equal{0};
inline constexpr strong_ordering strong_ordering::equivalent{0};
inline constexpr strong_ordering strong_ordering::greater{1};
}
int compare(int a, int b) { return (a <=> b).v; }
int main() { return compare(4, 9) == -1 ? 0 : 1; }
