// A port of Money.swift: a money type that counts cents, made from
// integer and decimal literals, with arithmetic and ordering. Exits 0
// when every check holds.

namespace std {
class strong_ordering {
  signed char v;  // -1, 0, or 1
public:
  constexpr explicit strong_ordering(signed char x) : v(x) {}
  static const strong_ordering less, equal, equivalent, greater;
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

// A short text of fixed capacity, for the description.
struct Text {
  char chars[32] = {};
  int length = 0;
  constexpr void add(char c) { chars[length++] = c; }
  constexpr void add(const char* s) {
    while (*s) add(*s++);
  }
  constexpr void add(long long n) {  // n >= 0
    char digits[20] = {};
    int k = 0;
    do {
      digits[k++] = static_cast<char>('0' + n % 10);
      n /= 10;
    } while (n != 0);
    while (k > 0) add(digits[--k]);
  }
  constexpr bool operator==(const char* s) const {
    for (int i = 0; i < length; ++i)
      if (s[i] != chars[i]) return false;
    return s[length] == '\0';
  }
};

template <class T, int Capacity>
struct List {
  T items[Capacity] = {};
  int count = 0;
  constexpr void append(T value) { items[count++] = value; }
  constexpr T* begin() { return items; }
  constexpr T* end() { return items + count; }
  constexpr const T* begin() const { return items; }
  constexpr const T* end() const { return items + count; }
};

struct Money {
  long long cents = 0;

  constexpr Money() = default;
  constexpr explicit Money(long long c) : cents(c) {}

  friend constexpr Money operator+(Money a, Money b) { return Money(a.cents + b.cents); }
  friend constexpr Money operator-(Money a, Money b) { return Money(a.cents - b.cents); }
  friend constexpr Money operator*(Money a, long long n) { return Money(a.cents * n); }
  constexpr auto operator<=>(const Money&) const = default;

  // Defined after the class, where Money is complete (FN-12).
  constexpr List<Money, 16> split(long long parts) const;

  constexpr Text description() const {
    Text text;
    if (cents < 0) text.add('-');
    long long magnitude = cents < 0 ? -cents : cents;
    long long rest = magnitude % 100;
    text.add('$');
    text.add(magnitude / 100);
    text.add('.');
    if (rest < 10) text.add('0');
    text.add(rest);
    return text;
  }
};

constexpr List<Money, 16> Money::split(long long parts) const {
  long long share = cents / parts;
  long long extra = cents % parts;
  List<Money, 16> result;
  for (long long i = 0; i < parts; ++i) result.append(Money(share + (i < extra ? 1 : 0)));
  return result;
}

// Literals: 2_usd is two dollars, 19.99_usd is 1999 cents.
constexpr Money operator""_usd(unsigned long long dollars) {
  return Money(static_cast<long long>(dollars) * 100);
}
constexpr Money operator""_usd(long double dollars) {
  return Money(static_cast<long long>(dollars * 100 + 0.5L));  // rounds; dollars >= 0
}

template <class T> constexpr T maximum(T a, T b) { return a < b ? b : a; }

template <class T, int N>
constexpr List<T, N> sorted(List<T, N> list) {
  for (int i = 1; i < list.count; ++i)
    for (int j = i; j > 0 && list.items[j] < list.items[j - 1]; --j) {
      T moved = list.items[j];
      list.items[j] = list.items[j - 1];
      list.items[j - 1] = moved;
    }
  return list;
}

static_assert((19.99_usd).cents == 1999);
static_assert(2_usd < 19.99_usd);

int failures = 0;
void check(bool ok) {
  if (!ok) ++failures;
}

int main() {
  Money price = 19.99_usd;
  Money fee = 2_usd;
  check(price.cents == 1999);
  check(fee.cents == 200);

  Money total = price * 3 + fee;
  check(total.cents == 6197);
  check(total.description() == "$61.97");
  check((fee - price).description() == "-$17.99");

  auto shares = Money(1000).split(3);
  check(shares.count == 3 && shares.items[0].cents == 334 && shares.items[1].cents == 333 &&
        shares.items[2].cents == 333);
  Money sum = 0_usd;
  for (Money share : shares) sum = sum + share;
  check(sum == 10_usd);

  List<Money, 3> unsorted;
  unsorted.append(Money(500));
  unsorted.append(1.5_usd);
  unsorted.append(3_usd);
  auto ordered = sorted(unsorted);
  check(ordered.items[0].cents == 150 && ordered.items[1].cents == 300 &&
        ordered.items[2].cents == 500);
  check(maximum(price, 20_usd) == 20_usd);

  return failures == 0 ? 0 : 1;
}
