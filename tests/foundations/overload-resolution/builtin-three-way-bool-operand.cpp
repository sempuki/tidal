// tidal-rules: OVL-11 OVL-9
// tidal-expect: reject
// tidal-error-clang: invalid operands to binary expression .*char.*bool
// tidal-error-gcc: invalid operands of types .char. and .bool. to binary .operator<=>.
// tidal-error-edg: invalid types
// tidal-note: The candidate (int, int) wins with a promotion on each side. The operands then convert by their conversion functions only, to char and bool, and the built-in <=> does not take a bool with a char.
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
struct ToChar { operator char() const; };
struct ToBool { operator bool() const; };
auto r = ToChar{} <=> ToBool{};
