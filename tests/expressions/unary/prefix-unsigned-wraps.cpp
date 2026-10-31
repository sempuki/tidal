// tidal-rules: UN-20
// tidal-expect: accept
constexpr bool f() {
  unsigned top = 4294967295u;
  ++top;
  unsigned low = 0u;
  --low;
  unsigned char small = 255;
  ++small;
  return top == 0u && low == 4294967295u && small == 0;
}
static_assert(f());
