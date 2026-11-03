// tidal-rules: CTOR-2
// tidal-expect: accept
// tidal-note: Write report W7. A constructor whose parameters all have default arguments is the default constructor.
struct Rational {
  long long num, den;
  constexpr Rational(long long n = 0, long long d = 1) : num(n), den(d) {}
};
struct Sum { Rational total; };
constexpr bool check() {
  Rational plain;
  Rational braced{};
  Rational list[2];
  Sum s;
  return plain.den == 1 && braced.num == 0 && list[1].den == 1 && s.total.den == 1;
}
static_assert(check());
