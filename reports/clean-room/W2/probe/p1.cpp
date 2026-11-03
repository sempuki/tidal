// Ordering types with only what LSUP-2 lists: static members, nothing else.
namespace std {
struct strong_ordering {
  int v;
  static const strong_ordering less, equal, equivalent, greater;
};
inline constexpr strong_ordering strong_ordering::less{-1};
inline constexpr strong_ordering strong_ordering::equal{0};
inline constexpr strong_ordering strong_ordering::equivalent{0};
inline constexpr strong_ordering strong_ordering::greater{1};
}
struct Date { int y, m, d; auto operator<=>(const Date&) const = default; };
static_assert(Date{2026,1,2} < Date{2026,1,3});
int main() { return 0; }
