// User-defined literals for a distance type, from an integer and a floating literal.
struct Meters { double v; };
constexpr Meters operator""_km(unsigned long long n) { return {static_cast<double>(n) * 1000.0}; }
constexpr Meters operator""_m(long double x) { return {static_cast<double>(x)}; }
constexpr Meters operator+(Meters a, Meters b) { return {a.v + b.v}; }
int main() {
  constexpr Meters d = 2_km + 12.5_m;
  static_assert(d.v == 2012.5);
  return d.v == 2012.5 ? 0 : 1;
}
