// tidal-rules: PFX-12 UN-20
// tidal-expect: accept
constexpr bool up() {
  unsigned top = 4294967295u;
  unsigned old = top++;
  unsigned char small = 255;
  unsigned char before = small++;
  return old == 4294967295u && top == 0u && before == 255 && small == 0;
}
constexpr bool down() {
  unsigned low = 0u;
  unsigned old = low--;
  return old == 0u && low == 4294967295u;
}
static_assert(up() && down());
