// tidal-rules: ASN-5
// tidal-expect: accept
constexpr int f() {
  int a = 20;
  a += 1; a -= 3; a *= 2; a /= 4; a %= 5;
  a <<= 3; a >>= 1; a &= 12; a |= 1; a ^= 3;
  return a;
}
static_assert(f() == 2);
