// tidal-rules: LSUP-2
// tidal-expect: reject
// tidal-error-clang: std::partial_ordering. was not found
// tidal-error-gcc: partial_ordering. is not a member of .std
// tidal-error-edg: no valid std::partial_ordering type found
// tidal-note: The program declares only std::strong_ordering. A <=> on two double operands yields std::partial_ordering, which the program does not declare, so the use is rejected.
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
bool compare(double a, double b) { (void)(a <=> b); return true; }
