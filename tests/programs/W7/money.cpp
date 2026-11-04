// tidal-rules: OPR-15 CTOR-9 ACC-7 DCMP-3 DCMP-4 CPT-24 CPT-26
// tidal-expect: run
// tidal-note: Write suite W7, program money.cpp (reports/clean-room/W7/report.md). The rules are the ones in the report's construct-to-rule notes that the program's comments name.
// tidal-note: A program test is evidence from use and decides no rule alone (decision 0054).
// tidal-note: The program text adds weak_ordering and partial_ordering in namespace std. LSUP-2 needs all three ordering classes, and EDG 7.0 rejects the program without them.
// W7: a money type that refuses mixed currencies, written from the
// Tidal specification. The currency is part of the type, so a mix
// has no operator to call (OPR-15).
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
// Added for the program test: LSUP-2 needs all three ordering classes,
// and EDG 7.0 rejects a program that declares only strong_ordering.
struct weak_ordering {
  signed char value;
  static const weak_ordering less;
  static const weak_ordering equivalent;
  static const weak_ordering greater;
};
inline constexpr weak_ordering weak_ordering::less{-1};
inline constexpr weak_ordering weak_ordering::equivalent{0};
inline constexpr weak_ordering weak_ordering::greater{1};
struct partial_ordering {
  signed char value;
  static const partial_ordering less;
  static const partial_ordering equivalent;
  static const partial_ordering greater;
  static const partial_ordering unordered;
};
inline constexpr partial_ordering partial_ordering::less{-1};
inline constexpr partial_ordering partial_ordering::equivalent{0};
inline constexpr partial_ordering partial_ordering::greater{1};
inline constexpr partial_ordering partial_ordering::unordered{2};
}

enum class Currency { usd, eur, jpy };

template <Currency C>
class Money {
  long long minor_;   // cents, or yen for jpy

public:
  // CTOR-9: explicit, so a bare number never becomes money by accident.
  explicit constexpr Money(long long minor) : minor_(minor) {}
  constexpr Money() : minor_(0) {}

  constexpr long long minor_units() const { return minor_; }
  static constexpr Currency currency() { return C; }

  constexpr Money& operator+=(Money o) { minor_ += o.minor_; return *this; }
  constexpr Money& operator-=(Money o) { minor_ -= o.minor_; return *this; }
  constexpr Money& operator*=(long long k) { minor_ *= k; return *this; }

  constexpr Money operator-() const { return Money(-minor_); }

  // ACC-7: hidden friends. Each takes two Money<C> of one C.
  friend constexpr Money operator+(Money a, Money b) { return a += b; }
  friend constexpr Money operator-(Money a, Money b) { return a -= b; }
  friend constexpr Money operator*(Money a, long long k) { return a *= k; }
  friend constexpr Money operator*(long long k, Money a) { return a *= k; }

  // DCMP-3, DCMP-4: one defaulted <=> gives all six comparisons.
  friend auto operator<=>(const Money&, const Money&) = default;

  // A converting constructor from another currency would let a mix through.
  // DEL: delete it by name so the mistake reads as a deleted function.
  template <Currency D>
  Money(Money<D>) = delete;
};

using Usd = Money<Currency::usd>;
using Eur = Money<Currency::eur>;

constexpr Usd five(500);
constexpr Usd two(200);

static_assert(five + two == Usd(700));
static_assert(five - two == Usd(300));
static_assert(two * 3 == Usd(600));
static_assert(3 * two == Usd(600));
static_assert(-two == Usd(-200));
static_assert(two < five);
static_assert(five >= five);
static_assert(Usd{} == Usd(0));
static_assert(Usd::currency() == Currency::usd);

// CPT-24, CPT-26: test what the type refuses, without making the program
// invalid. Each concept gives false when the expression is not valid.
template <class A, class B> concept Addable = requires(A a, B b) { a + b; };
template <class A, class B> concept Comparable = requires(A a, B b) { a == b; };
template <class A, class B> concept LessComparable = requires(A a, B b) { a < b; };
template <class A, class B> concept Assignable = requires(A a, B b) { a = b; };
template <class T> concept FromNumber = requires(void (*f)(T)) { f(5); };

static_assert(Addable<Usd, Usd>);
static_assert(!Addable<Usd, Eur>);          // OPR-15: no viable operator
static_assert(!Comparable<Usd, Eur>);
static_assert(!LessComparable<Usd, Eur>);
static_assert(!Assignable<Usd&, Eur>);      // deleted converting constructor
static_assert(!Addable<Usd, int>);          // CTOR-9: 5 does not become Usd
static_assert(!FromNumber<Usd>);

// Runtime totals: sum a list of prices in one currency.
template <Currency C>
constexpr Money<C> total(const Money<C>* items, int n) {
  Money<C> sum;
  for (int i = 0; i < n; ++i) sum += items[i];
  return sum;
}

int main() {
  Eur cart[3] = {Eur(1999), Eur(501), Eur(2500)};
  Eur t = total(cart, 3);
  if (t != Eur(5000)) return 1;
  if (!(t > cart[0])) return 2;
  Usd u(100);
  u *= 7;
  if (u.minor_units() != 700) return 3;
  return 0;
}
