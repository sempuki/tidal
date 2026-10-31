// tidal-rules: DCMP-5
// tidal-expect: reject
namespace std {
struct partial_ordering {
  signed char value;
  static const partial_ordering less, equivalent, greater, unordered;
  friend constexpr bool operator==(partial_ordering, partial_ordering) = default;
  friend constexpr bool operator==(partial_ordering v, int) { return v.value == 0; }
  friend constexpr bool operator<(partial_ordering v, int) { return v.value == -1; }
  friend constexpr bool operator>(partial_ordering v, int) { return v.value == 1; }
  friend constexpr bool operator<=(partial_ordering v, int) { return v.value == -1 || v.value == 0; }
  friend constexpr bool operator>=(partial_ordering v, int) { return v.value == 1 || v.value == 0; }
  friend constexpr bool operator<(int, partial_ordering v) { return v.value == 1; }
  friend constexpr bool operator>(int, partial_ordering v) { return v.value == -1; }
  friend constexpr bool operator<=(int, partial_ordering v) { return v.value == 1 || v.value == 0; }
  friend constexpr bool operator>=(int, partial_ordering v) { return v.value == -1 || v.value == 0; }
};
inline constexpr partial_ordering partial_ordering::less{-1};
inline constexpr partial_ordering partial_ordering::equivalent{0};
inline constexpr partial_ordering partial_ordering::greater{1};
inline constexpr partial_ordering partial_ordering::unordered{2};
struct weak_ordering {
  signed char value;
  static const weak_ordering less, equivalent, greater;
  constexpr operator partial_ordering() const { return {value}; }
  friend constexpr bool operator==(weak_ordering, weak_ordering) = default;
  friend constexpr bool operator==(weak_ordering v, int) { return v.value == 0; }
  friend constexpr bool operator<(weak_ordering v, int) { return v.value < 0; }
  friend constexpr bool operator>(weak_ordering v, int) { return v.value > 0; }
  friend constexpr bool operator<=(weak_ordering v, int) { return v.value <= 0; }
  friend constexpr bool operator>=(weak_ordering v, int) { return v.value >= 0; }
  friend constexpr bool operator<(int, weak_ordering v) { return 0 < v.value; }
  friend constexpr bool operator>(int, weak_ordering v) { return 0 > v.value; }
  friend constexpr bool operator<=(int, weak_ordering v) { return 0 <= v.value; }
  friend constexpr bool operator>=(int, weak_ordering v) { return 0 >= v.value; }
};
inline constexpr weak_ordering weak_ordering::less{-1};
inline constexpr weak_ordering weak_ordering::equivalent{0};
inline constexpr weak_ordering weak_ordering::greater{1};
struct strong_ordering {
  signed char value;
  static const strong_ordering less, equal, equivalent, greater;
  constexpr operator partial_ordering() const { return {value}; }
  constexpr operator weak_ordering() const { return {value}; }
  friend constexpr bool operator==(strong_ordering, strong_ordering) = default;
  friend constexpr bool operator==(strong_ordering v, int) { return v.value == 0; }
  friend constexpr bool operator<(strong_ordering v, int) { return v.value < 0; }
  friend constexpr bool operator>(strong_ordering v, int) { return v.value > 0; }
  friend constexpr bool operator<=(strong_ordering v, int) { return v.value <= 0; }
  friend constexpr bool operator>=(strong_ordering v, int) { return v.value >= 0; }
  friend constexpr bool operator<(int, strong_ordering v) { return 0 < v.value; }
  friend constexpr bool operator>(int, strong_ordering v) { return 0 > v.value; }
  friend constexpr bool operator<=(int, strong_ordering v) { return 0 <= v.value; }
  friend constexpr bool operator>=(int, strong_ordering v) { return 0 >= v.value; }
};
inline constexpr strong_ordering strong_ordering::less{-1};
inline constexpr strong_ordering strong_ordering::equal{0};
inline constexpr strong_ordering strong_ordering::equivalent{0};
inline constexpr strong_ordering strong_ordering::greater{1};
}
template <class A, class B> constexpr bool same = false;
template <class A> constexpr bool same<A, A> = true;
struct S { int v; bool operator<(const S&) const = default; };
bool r = S{1} < S{2};
