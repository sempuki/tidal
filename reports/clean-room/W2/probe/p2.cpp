// Add comparisons with a literal 0, as OVL-13 implies.
namespace std {
struct strong_ordering {
  int v;
  static const strong_ordering less, equal, equivalent, greater;
  friend constexpr bool operator==(strong_ordering, strong_ordering) = default;
  friend constexpr bool operator==(strong_ordering a, int) { return a.v == 0; }
  friend constexpr bool operator<(strong_ordering a, int) { return a.v < 0; }
  friend constexpr bool operator<(int, strong_ordering a) { return 0 < a.v; }
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
struct Date { int y, m, d; auto operator<=>(const Date&) const = default; };
static_assert(Date{2026,1,2} < Date{2026,1,3});
static_assert(Date{2026,1,2} == Date{2026,1,2});
static_assert(!(Date{2026,1,2} > Date{2026,1,3}));
static_assert((Date{1,1,1} <=> Date{1,1,1}) == std::strong_ordering::equal);
struct Reading { int id; double v; auto operator<=>(const Reading&) const = default; };
bool f(Reading a, Reading b) { return a < b; }
int main() { return 0; }
