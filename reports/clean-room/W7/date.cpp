// W7: a date with validation, written from the Tidal specification.
// LSUP-2: a defaulted <=> needs the ordering classes declared in std.
namespace std {
struct strong_ordering {
  signed char value;
  static const strong_ordering less;
  static const strong_ordering equal;
  static const strong_ordering equivalent;
  static const strong_ordering greater;
  friend constexpr bool operator==(strong_ordering a, strong_ordering b) { return a.value == b.value; }
  friend constexpr bool operator==(strong_ordering a, int) { return a.value == 0; }
  friend constexpr bool operator<(strong_ordering a, int) { return a.value < 0; }
  friend constexpr bool operator>(strong_ordering a, int) { return a.value > 0; }
  friend constexpr bool operator<=(strong_ordering a, int) { return a.value <= 0; }
  friend constexpr bool operator>=(strong_ordering a, int) { return a.value >= 0; }
  friend constexpr bool operator<(int, strong_ordering a) { return 0 < a.value; }
  friend constexpr bool operator>(int, strong_ordering a) { return 0 > a.value; }
  friend constexpr bool operator<=(int, strong_ordering a) { return 0 <= a.value; }
  friend constexpr bool operator>=(int, strong_ordering a) { return 0 >= a.value; }
};
inline constexpr strong_ordering strong_ordering::less{-1};
inline constexpr strong_ordering strong_ordering::equal{0};
inline constexpr strong_ordering strong_ordering::equivalent{0};
inline constexpr strong_ordering strong_ordering::greater{1};
}

// A scoped enumeration keeps a month from mixing with a day (CMP-23).
enum class Month : int { jan = 1, feb, mar, apr, may, jun, jul, aug, sep, oct, nov, dec };

class Date {
  // DCMP-4: a defaulted <=> compares members in declaration order,
  // so year, month, day gives calendar order.
  int year_;
  Month month_;
  int day_;

public:
  struct Invalid {};

  static constexpr bool is_leap(int y) {                       // CLS-7
    return (y % 4 == 0 && y % 100 != 0) || y % 400 == 0;
  }
  static constexpr int days_in(int y, Month m) {
    switch (m) {
      case Month::feb: return is_leap(y) ? 29 : 28;
      case Month::apr: case Month::jun: case Month::sep: case Month::nov: return 30;
      default: return 31;
    }
  }
  static constexpr bool valid(int y, int m, int d) {
    return y >= 1 && m >= 1 && m <= 12 && d >= 1 && d <= days_in(y, static_cast<Month>(m));
  }

  // CTOR-9: explicit, so Date d = {2024, Month::jan, 1}; is not allowed.
  // CTOR-2: this constructor removes the implicit default constructor.
  explicit constexpr Date(int y, Month m, int d) : year_(y), month_(m), day_(d) {
    if (!valid(y, static_cast<int>(m), d)) throw Invalid{};
  }

  constexpr int year() const { return year_; }
  constexpr Month month() const { return month_; }
  constexpr int day() const { return day_; }

  // DCMP-3, DCMP-4: also declares a defaulted ==.
  auto operator<=>(const Date&) const = default;

  // OPR-4: prefix form moves to the next day.
  constexpr Date& operator++() {
    if (day_ < days_in(year_, month_)) {
      ++day_;
    } else if (month_ != Month::dec) {
      day_ = 1;
      month_ = static_cast<Month>(static_cast<int>(month_) + 1);
    } else {
      day_ = 1;
      month_ = Month::jan;
      ++year_;
    }
    return *this;
  }
  // OPR-4: postfix form takes a dummy int.
  constexpr Date operator++(int) {
    Date old = *this;
    ++*this;
    return old;
  }
};

constexpr Date leap_day(2024, Month::feb, 29);

static_assert(Date::valid(2024, 2, 29));
static_assert(!Date::valid(2023, 2, 29));
static_assert(!Date::valid(1900, 2, 29));
static_assert(Date::valid(2000, 2, 29));
static_assert(!Date::valid(2024, 13, 1));
static_assert(!Date::valid(2024, 4, 31));
static_assert(!Date::valid(2024, 1, 0));

static_assert(Date(2024, Month::jan, 31) < Date(2024, Month::feb, 1));
static_assert(Date(2023, Month::dec, 31) < Date(2024, Month::jan, 1));
static_assert(Date(2024, Month::mar, 1) == Date(2024, Month::mar, 1));
static_assert(Date(2024, Month::mar, 1) != Date(2024, Month::mar, 2));
static_assert(++Date(leap_day) == Date(2024, Month::mar, 1));
static_assert(++Date(2023, Month::dec, 31) == Date(2024, Month::jan, 1));

constexpr Date after(Date d, int n) {
  while (n-- > 0) ++d;
  return d;
}
static_assert(after(Date(2024, Month::jan, 1), 366) == Date(2025, Month::jan, 1));

// CPT-24, CPT-26: a requires expression on template parameters reports
// whether a use is valid without making the program invalid.
template <class T> concept DefaultConstructible = requires { T{}; };
template <class T> concept FromBraces = requires(void (*f)(T)) { f({2024, Month::jan, 1}); };
static_assert(!DefaultConstructible<Date>);   // CTOR-2, CTOR-19
static_assert(!FromBraces<Date>);             // CTOR-9, ILC-8

int main() {
  int caught = 0;
  const int bad[][3] = {{2023, 2, 29}, {2024, 4, 31}, {2024, 1, 32}, {0, 1, 1}};
  for (const auto& b : bad) {
    try {
      Date d(b[0], static_cast<Month>(b[1]), b[2]);
      (void)d;
    } catch (const Date::Invalid&) {
      ++caught;
    }
  }
  if (caught != 4) return 1;

  Date d(2024, Month::feb, 28);
  Date before = d++;
  if (before != Date(2024, Month::feb, 28)) return 2;
  if (d != Date(2024, Month::feb, 29)) return 3;
  ++d;
  if (d.month() != Month::mar || d.day() != 1) return 4;
  if (!(before < d)) return 5;
  return 0;
}
