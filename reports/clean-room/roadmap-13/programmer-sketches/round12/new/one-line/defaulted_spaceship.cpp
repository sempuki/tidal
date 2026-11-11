// A defaulted <=> with my own std::strong_ordering.
namespace std {
struct strong_ordering {
  signed char v;
  static const strong_ordering less, equal, greater;
  friend constexpr bool operator==(strong_ordering, strong_ordering) = default;
  friend constexpr bool operator<(strong_ordering a, decltype(nullptr)) { return a.v < 0; }
  friend constexpr bool operator==(strong_ordering a, decltype(nullptr)) { return a.v == 0; }
};
inline constexpr strong_ordering strong_ordering::less{-1}, strong_ordering::equal{0}, strong_ordering::greater{1};
}
struct V { int major, minor; auto operator<=>(const V&) const = default; };
int main() { return (V{1, 2} < V{1, 3} && !(V{2, 0} < V{1, 9})) ? 0 : 1; }
