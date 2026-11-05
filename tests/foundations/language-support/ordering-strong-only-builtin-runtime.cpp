// tidal-rules: LSUP-2
// tidal-expect: reject
// tidal-expect-gcc: accept
// tidal-expect-clang: accept
// tidal-expect-edg: accept
// tidal-note: Q-0386. The form of ordering-strong-only-builtin.cpp whose <=> is evaluated at run time. Every compiler accepts the program, and EDG 7.0 checks for the other two classes only where it evaluates a built-in <=> as a constant.
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
