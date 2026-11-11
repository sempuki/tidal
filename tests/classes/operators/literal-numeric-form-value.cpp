// tidal-rules: OPR-8
// tidal-expect: accept
// tidal-note: Ledger row P4-7. The numeric form receives the value that the literal without its suffix has as a literal of type long double or unsigned long long: the rounding, the prefix, the exponent, and the digit separators are those of the plain literal. Every compiler accepts the program.
constexpr long double operator""_ld(long double v) { return v; }
constexpr unsigned long long operator""_ull(unsigned long long v) { return v; }
constexpr long long operator""_cents(long double v) {
  return static_cast<long long>(v * 100 + 0.5L);
}
static_assert(19.99_ld == 19.99L);
static_assert(0.1_ld == 0.1L);
static_assert(1e300_ld == 1e300L);
static_assert(0x1p-2_ld == 0.25L);
static_assert(1'000.5_ld == 1000.5L);
static_assert(19.99_cents == 1999);
static_assert(18446744073709551615_ull == 18446744073709551615ull);
static_assert(0x10_ull == 16);
static_assert(0b101_ull == 5);
static_assert(017_ull == 15);
static_assert(1'000_ull == 1000);
