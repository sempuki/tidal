// A fixed-point number type: Q16.16 in a 32-bit int.
// The raw value is the number times 65536.

constexpr int frac_bits = 16;
constexpr int one_raw = 1 << frac_bits;  // SHF-5: 65536

class Fixed {
public:
  // A Rust-style named constructor. The private constructor takes the
  // raw bits, so that Fixed(3) cannot mean "raw 3" by mistake.
  static constexpr Fixed from_raw(int raw) { return Fixed(raw, 0); }

  // SHF-5: a left shift of a negative value is defined, modulo 2^32.
  // Values outside -32768..32767 lose their high bits.
  static constexpr Fixed from_int(int v) { return Fixed(v << frac_bits, 0); }

  // n / d, rounded toward zero (MUL-6). The shift happens in long long,
  // so the numerator cannot lose bits (SHF-4: result type is the
  // promoted left operand).
  static constexpr Fixed ratio(int n, int d) {
    return Fixed(static_cast<int>((static_cast<long long>(n) << frac_bits) / d), 0);
  }

  // CONV-4: a floating value converts to int by dropping the fraction.
  // CONV-5: the value must fit in int.
  static constexpr Fixed from_double(double d) {
    return Fixed(static_cast<int>(d * one_raw), 0);
  }

  constexpr int raw() const { return raw_; }

  // SHF-6: >> rounds toward negative infinity, so this is floor().
  constexpr int floor() const { return raw_ >> frac_bits; }

  // MUL-6: / rounds toward zero, so this is trunc().
  constexpr int trunc() const { return raw_ / one_raw; }

  // TYP-9: int and double give double. CONV-6: every int is exact in
  // double (53 bits, TYP-10), and the division by a power of two is exact.
  constexpr double to_double() const { return raw_ / 65536.0; }

  // ADD-9: these overflow as int does, so the behavior is undefined
  // when a sum leaves the range. Rust would panic or wrap here. The
  // caller keeps values in range.
  constexpr Fixed operator+(Fixed o) const { return Fixed(raw_ + o.raw_, 0); }
  constexpr Fixed operator-(Fixed o) const { return Fixed(raw_ - o.raw_, 0); }
  constexpr Fixed operator-() const { return Fixed(-raw_, 0); }

  // The product of two raw values needs 64 bits (CONV-19 step 3).
  // SHF-6: the shift floors the result.
  constexpr Fixed operator*(Fixed o) const {
    long long wide = static_cast<long long>(raw_) * o.raw_;
    return Fixed(static_cast<int>(wide >> frac_bits), 0);
  }

  // SHF-5: the left shift of a negative long long is defined.
  // MUL-6: the quotient rounds toward zero. MUL-9: o must not be zero.
  constexpr Fixed operator/(Fixed o) const {
    long long wide = static_cast<long long>(raw_) << frac_bits;
    return Fixed(static_cast<int>(wide / o.raw_), 0);
  }

  // ASN-5 does not give a class +=. OPR-6: a @= b uses only operator@=.
  constexpr Fixed& operator+=(Fixed o) {
    raw_ += o.raw_;
    return *this;
  }

  constexpr bool operator==(Fixed o) const { return raw_ == o.raw_; }
  constexpr bool operator<(Fixed o) const { return raw_ < o.raw_; }

private:
  // The second parameter only makes this constructor distinct from a
  // public conversion from int.
  constexpr Fixed(int raw, int) : raw_(raw) {}
  int raw_;
};

constexpr Fixed operator""_fx(unsigned long long v) {  // OPR-8, OPR-10
  return Fixed::from_int(static_cast<int>(v));
}

constexpr Fixed half = Fixed::ratio(1, 2);
constexpr Fixed one_and_half = Fixed::ratio(3, 2);

static_assert(Fixed::from_int(1).raw() == 65536);
static_assert(Fixed::from_int(-1).raw() == -65536);
static_assert((1_fx + half).raw() == 98304);
static_assert(one_and_half * Fixed::ratio(9, 4) == Fixed::ratio(27, 8));
static_assert((3_fx / 2_fx) == one_and_half);
static_assert(Fixed::ratio(-3, 2).floor() == -2);  // floor
static_assert(Fixed::ratio(-3, 2).trunc() == -1);  // toward zero
static_assert(Fixed::ratio(7, 2).floor() == 3);
static_assert((-one_and_half).raw() == -98304);
static_assert(Fixed::from_double(2.75) == Fixed::ratio(11, 4));
static_assert(Fixed::from_double(-0.5) == -half);
static_assert(Fixed::ratio(1, 3).to_double() == 21845 / 65536.0);
static_assert(half < one_and_half);
static_assert(!(one_and_half < half));
static_assert(Fixed::from_raw(1) * Fixed::from_raw(1) == Fixed::from_raw(0));
// The smallest negative product floors to -1 raw unit, not to 0.
static_assert(Fixed::from_raw(-1) * Fixed::from_raw(1) == Fixed::from_raw(-1));
static_assert(Fixed::from_int(32767).floor() == 32767);
static_assert(Fixed::from_int(-32768).floor() == -32768);

constexpr Fixed sum_of_halves(int n) {
  Fixed s = 0_fx;
  for (int i = 0; i < n; ++i) s += half;
  return s;
}
static_assert(sum_of_halves(9) == Fixed::ratio(9, 2));

int main(int argc, char**) {
  Fixed a = Fixed::from_int(argc * 5);        // 5
  Fixed b = Fixed::ratio(argc, 4);            // 0.25
  if ((a * b).to_double() != 1.25) return 1;
  if ((a / b).floor() != 20) return 2;
  if ((b - a).floor() != -5) return 3;        // -4.75 floors to -5
  if ((b - a).trunc() != -4) return 4;
  Fixed c = Fixed::from_double(-1.0 / 3.0);
  if (c.raw() != -21845) return 5;            // CONV-4: toward zero
  if (!(c < b)) return 6;
  if (sum_of_halves(argc * 1000).floor() != 500) return 7;
  return 0;
}
