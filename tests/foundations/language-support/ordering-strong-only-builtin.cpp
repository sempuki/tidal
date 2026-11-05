// tidal-rules: LSUP-2
// tidal-expect: run
// tidal-expect-edg: reject
// tidal-error-edg: no valid std::weak_ordering type found
// tidal-note: Q-0386. The program declares std::strong_ordering and neither of the other two ordering classes. The <=> on two int operands yields std::strong_ordering, and its value is a constant. The use needs only the class it yields, so the program is valid. Clang and GCC accept it. EDG asks for the other two classes where it evaluates a built-in <=> as a constant.
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
constexpr std::strong_ordering r = 4 <=> 9;
int main() { return r.v == -1 ? 0 : 1; }
