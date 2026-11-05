namespace std {
struct strong_ordering {
  signed char v;
  static const strong_ordering less, equal, equivalent, greater;
  friend constexpr bool operator!=(strong_ordering a, int) { return a.v != 0; }
  friend constexpr bool operator==(strong_ordering a, int) { return a.v == 0; }
};
inline constexpr strong_ordering strong_ordering::less{-1};
inline constexpr strong_ordering strong_ordering::equal{0};
inline constexpr strong_ordering strong_ordering::equivalent{0};
inline constexpr strong_ordering strong_ordering::greater{1};
}
struct P { int x; auto operator<=>(const P&) const = default; };
auto r = P{1} <=> P{2};
int main() { return r.v == -1 ? 0 : 1; }
