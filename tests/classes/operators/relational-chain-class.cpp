// tidal-rules: OPR-11 OVL-13
// tidal-expect: run
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
struct Less {                    // has its own operator<
  int v;
  Less(int x) : v(x) {}
};
bool operator<(Less a, Less b) { return a.v < b.v; }
struct Spaceship {               // has only a defaulted <=>
  int v;
  Spaceship(int x) : v(x) {}
  auto operator<=>(const Spaceship&) const = default;
};
int main() {
  bool a = Less{3} < Less{2} < Less{1};               // (false) < Less{1}: Less(0) < Less(1)
  bool b = Spaceship{3} < Spaceship{2} < Spaceship{1}; // 0 < (Spaceship{1} <=> Spaceship(0))
  bool c = Less{1} < Less{2} < Less{0};
  return a && b && !c ? 0 : 1;
}
