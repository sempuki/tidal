// tidal-rules: LSUP-2 LSUP-12 LSUP-13
// tidal-expect: accept
// tidal-note: Write reports W2 and W7. The three ordering classes with every part that the implementation uses.
namespace std {
class partial_ordering {
  signed char v;  // -1, 0, 1, or 2 for unordered
public:
  constexpr explicit partial_ordering(signed char x) : v(x) {}
  static const partial_ordering less, equivalent, greater, unordered;
  friend constexpr bool operator==(partial_ordering, partial_ordering) = default;
  friend constexpr bool operator==(partial_ordering a, int) { return a.v == 0; }
  friend constexpr bool operator<(partial_ordering a, int) { return a.v == -1; }
  friend constexpr bool operator>(partial_ordering a, int) { return a.v == 1; }
  friend constexpr bool operator<=(partial_ordering a, int) { return a.v == -1 || a.v == 0; }
  friend constexpr bool operator>=(partial_ordering a, int) { return a.v == 1 || a.v == 0; }
  friend constexpr bool operator<(int, partial_ordering a) { return a > 0; }
  friend constexpr bool operator>(int, partial_ordering a) { return a < 0; }
  friend constexpr bool operator<=(int, partial_ordering a) { return a >= 0; }
  friend constexpr bool operator>=(int, partial_ordering a) { return a <= 0; }
  friend constexpr partial_ordering operator<=>(int, partial_ordering a) {
    return partial_ordering(a.v == 2 ? 2 : -a.v);
  }
};
inline constexpr partial_ordering partial_ordering::less{-1};
inline constexpr partial_ordering partial_ordering::equivalent{0};
inline constexpr partial_ordering partial_ordering::greater{1};
inline constexpr partial_ordering partial_ordering::unordered{2};

class weak_ordering {
  signed char v;  // -1, 0, or 1
public:
  constexpr explicit weak_ordering(signed char x) : v(x) {}
  static const weak_ordering less, equivalent, greater;
  constexpr operator partial_ordering() const { return partial_ordering(v); }
  friend constexpr bool operator==(weak_ordering, weak_ordering) = default;
  friend constexpr bool operator==(weak_ordering a, int) { return a.v == 0; }
  friend constexpr bool operator<(weak_ordering a, int) { return a.v < 0; }
  friend constexpr bool operator>(weak_ordering a, int) { return a.v > 0; }
  friend constexpr bool operator<=(weak_ordering a, int) { return a.v <= 0; }
  friend constexpr bool operator>=(weak_ordering a, int) { return a.v >= 0; }
  friend constexpr bool operator<(int, weak_ordering a) { return a.v > 0; }
  friend constexpr bool operator>(int, weak_ordering a) { return a.v < 0; }
  friend constexpr bool operator<=(int, weak_ordering a) { return a.v >= 0; }
  friend constexpr bool operator>=(int, weak_ordering a) { return a.v <= 0; }
  friend constexpr weak_ordering operator<=>(int, weak_ordering a) { return weak_ordering(-a.v); }
};
inline constexpr weak_ordering weak_ordering::less{-1};
inline constexpr weak_ordering weak_ordering::equivalent{0};
inline constexpr weak_ordering weak_ordering::greater{1};

class strong_ordering {
  signed char v;  // -1, 0, or 1
public:
  constexpr explicit strong_ordering(signed char x) : v(x) {}
  static const strong_ordering less, equal, equivalent, greater;
  constexpr operator partial_ordering() const { return partial_ordering(v); }
  constexpr operator weak_ordering() const { return weak_ordering(v); }
  friend constexpr bool operator==(strong_ordering, strong_ordering) = default;
  friend constexpr bool operator==(strong_ordering a, int) { return a.v == 0; }
  friend constexpr bool operator<(strong_ordering a, int) { return a.v < 0; }
  friend constexpr bool operator>(strong_ordering a, int) { return a.v > 0; }
  friend constexpr bool operator<=(strong_ordering a, int) { return a.v <= 0; }
  friend constexpr bool operator>=(strong_ordering a, int) { return a.v >= 0; }
  friend constexpr bool operator<(int, strong_ordering a) { return a.v > 0; }
  friend constexpr bool operator>(int, strong_ordering a) { return a.v < 0; }
  friend constexpr bool operator<=(int, strong_ordering a) { return a.v >= 0; }
  friend constexpr bool operator>=(int, strong_ordering a) { return a.v <= 0; }
  friend constexpr strong_ordering operator<=>(int, strong_ordering a) { return strong_ordering(-a.v); }
};
inline constexpr strong_ordering strong_ordering::less{-1};
inline constexpr strong_ordering strong_ordering::equal{0};
inline constexpr strong_ordering strong_ordering::equivalent{0};
inline constexpr strong_ordering strong_ordering::greater{1};
}

// Built-in operands.
static_assert((1 <=> 2) == std::strong_ordering::less);
static_assert((1.0 <=> 2.0) == std::partial_ordering::less);

// A defaulted <=> tests each part with == 0 and converts it to partial_ordering.
struct Reading {
  int sensor;
  double value;
  auto operator<=>(const Reading&) const = default;
};
static_assert((Reading{1, 2.0} <=> Reading{1, 3.0}) == std::partial_ordering::less);
static_assert((Reading{2, 0.0} <=> Reading{1, 9.0}) == std::partial_ordering::greater);
static_assert((Reading{1, 2.0} <=> Reading{1, 2.0}) == std::partial_ordering::equivalent);
static_assert(Reading{1, 2.0} < Reading{1, 3.0});

// A weak member and a strong member give weak_ordering.
struct Loose {
  int key;
  constexpr std::weak_ordering operator<=>(const Loose& o) const { return key <=> o.key; }
  constexpr bool operator==(const Loose& o) const { return key == o.key; }
};
struct Entry {
  Loose name;
  int rank;
  auto operator<=>(const Entry&) const = default;
};
static_assert((Entry{{1}, 2} <=> Entry{{1}, 3}) == std::weak_ordering::less);

// The four relational operators, rewritten from <=> with 0 on the right.
struct Date {
  int year, month;
  auto operator<=>(const Date&) const = default;
};
static_assert(Date{2024, 1} < Date{2024, 2});
static_assert(Date{2025, 1} > Date{2024, 2});
static_assert(Date{2024, 1} <= Date{2024, 1});
static_assert(Date{2024, 3} >= Date{2024, 2});

// A <=> declared only for (Meter, int): the reversed candidates put 0 on the left.
struct Meter {
  int value;
  friend constexpr std::strong_ordering operator<=>(Meter m, int i) { return m.value <=> i; }
};
static_assert(Meter{3} < 5);
static_assert(5 > Meter{3});
static_assert(5 >= Meter{5});
static_assert((5 <=> Meter{3}) == std::strong_ordering::greater);
