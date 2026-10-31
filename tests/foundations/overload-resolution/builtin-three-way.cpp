// tidal-rules: OVL-9
// tidal-expect: accept
// tidal-note: The built-in <=> candidates give std::strong_ordering for integer operands and std::partial_ordering for floating-point operands. The test declares the two types itself.
template <class T, class U> constexpr bool same = false;
template <class T> constexpr bool same<T, T> = true;
namespace std {
struct strong_ordering {
  signed char v;
  static const strong_ordering less, equal, equivalent, greater;
};
inline constexpr strong_ordering strong_ordering::less{-1};
inline constexpr strong_ordering strong_ordering::equal{0};
inline constexpr strong_ordering strong_ordering::equivalent{0};
inline constexpr strong_ordering strong_ordering::greater{1};
struct partial_ordering {
  signed char v;
  static const partial_ordering less, equivalent, greater, unordered;
};
inline constexpr partial_ordering partial_ordering::less{-1};
inline constexpr partial_ordering partial_ordering::equivalent{0};
inline constexpr partial_ordering partial_ordering::greater{1};
inline constexpr partial_ordering partial_ordering::unordered{2};
}
struct ToInt { operator int() const; };
struct ToDouble { operator double() const; };
static_assert(same<decltype(ToInt{} <=> 1), std::strong_ordering>);
static_assert(same<decltype(ToDouble{} <=> 1.0), std::partial_ordering>);
