// tidal-rules: BF-5
// tidal-expect: run
// tidal-note: Bit-fields of signed char and long long hold the most negative value of their width.
struct S { signed char small : 4; long long big : 5; };
int main() {
  S s{};
  s.small = -8;
  s.big = -16;
  return s.small == -8 && s.big == -16 ? 0 : 1;
}
