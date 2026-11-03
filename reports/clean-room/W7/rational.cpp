// W7: a rational number, written from the Tidal specification.
// LSUP-2: a program that uses <=> on built-in operands must declare the
// ordering classes in namespace std first.
namespace std {
struct strong_ordering {
  signed char value;
  static const strong_ordering less;
  static const strong_ordering equal;
  static const strong_ordering equivalent;
  static const strong_ordering greater;
  // OVL-13: x < y can become (x <=> y) < 0, so the ordering needs
  // comparisons against the literal 0.
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

class Rational {
  long long num_;   // ACC-1: private by default in a class
  long long den_;   // always > 0, and gcd(num_, den_) == 1

  static constexpr long long abs_(long long v) { return v < 0 ? -v : v; }
  static constexpr long long gcd_(long long a, long long b) {
    a = abs_(a);
    b = abs_(b);
    while (b != 0) {
      long long t = a % b;
      a = b;
      b = t;
    }
    return a;
  }

public:
  struct ZeroDenominator {};

  // CTOR-10: not explicit, so an int converts to a Rational.
  // CTOR-6: members are built in declaration order.
  constexpr Rational(long long n = 0, long long d = 1) : num_(n), den_(d) {
    if (den_ == 0) throw ZeroDenominator{};
    if (den_ < 0) { num_ = -num_; den_ = -den_; }
    long long g = gcd_(num_, den_);
    num_ /= g;
    den_ /= g;
  }

  constexpr long long num() const { return num_; }   // CLS-5
  constexpr long long den() const { return den_; }

  // OPR-6: compound assignment is never synthesized, so write it.
  constexpr Rational& operator+=(const Rational& o) {
    *this = Rational(num_ * o.den_ + o.num_ * den_, den_ * o.den_);
    return *this;
  }
  constexpr Rational& operator-=(const Rational& o) {
    *this = Rational(num_ * o.den_ - o.num_ * den_, den_ * o.den_);
    return *this;
  }
  constexpr Rational& operator*=(const Rational& o) {
    *this = Rational(num_ * o.num_, den_ * o.den_);
    return *this;
  }
  constexpr Rational& operator/=(const Rational& o) {
    *this = Rational(num_ * o.den_, den_ * o.num_);
    return *this;
  }

  constexpr Rational operator-() const { return Rational(-num_, den_); }

  // ACC-7, OPR-6: hidden friends are non-members, so both operands convert.
  friend constexpr Rational operator+(Rational a, const Rational& b) { return a += b; }
  friend constexpr Rational operator-(Rational a, const Rational& b) { return a -= b; }
  friend constexpr Rational operator*(Rational a, const Rational& b) { return a *= b; }
  friend constexpr Rational operator/(Rational a, const Rational& b) { return a /= b; }

  // DCMP-3, DCMP-1: memberwise == is right because the form is normalized.
  friend constexpr bool operator==(const Rational&, const Rational&) = default;

  // DCMP-6: <, >, <=, >= come from this <=> through rewriting.
  friend constexpr std::strong_ordering operator<=>(const Rational& a, const Rational& b) {
    return a.num_ * b.den_ <=> b.num_ * a.den_;
  }

  // CVF-3: explicit conversions, usable by static_cast and in conditions.
  explicit constexpr operator double() const { return double(num_) / double(den_); }
  explicit constexpr operator bool() const { return num_ != 0; }
};

constexpr Rational half(1, 2);
constexpr Rational third(1, 3);

static_assert(half + third == Rational(5, 6));
static_assert(Rational(2, 4) == half);
static_assert(Rational(3, -6) == -half);
static_assert(half - half == 0);              // 0 converts (CTOR-10)
static_assert(1 + half == Rational(3, 2));    // left operand converts (OPR-6)
static_assert(half * 2 == 1);
static_assert(half / third == Rational(3, 2));
static_assert(third < half);
static_assert(half > third);
static_assert(half <= half && half >= third);
static_assert(1 > half);                       // reversed: 0 > (half <=> 1)
static_assert((third <=> half) == std::strong_ordering::less);
static_assert(static_cast<double>(half) == 0.5);
static_assert(half);                           // contextual conversion (CVF-3)
static_assert(!Rational(0, 5));
static_assert(Rational(0, 5).den() == 1);

constexpr bool bad_zero_denominator() {
  try {
    Rational r(1, 0);
    (void)r;
  } catch (Rational::ZeroDenominator) {
    return true;
  }
  return false;
}

int main() {
  Rational sum;
  for (int i = 1; i <= 4; ++i) sum += Rational(1, i * (i + 1));
  if (sum != Rational(4, 5)) return 1;          // DCMP-6: !(sum == ...)
  Rational r = 3;                                // copy-init through converting ctor
  r /= Rational(9, 2);
  if (r != Rational(2, 3)) return 2;
  if (!bad_zero_denominator()) return 3;
  double d = static_cast<double>(Rational(-3, 4));
  if (d != -0.75) return 4;
  return 0;
}
