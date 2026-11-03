// tidal-rules: SB-10
// tidal-expect: run
// tidal-note: The binding refers to an unsigned bit-field of 3 bits. A value that does not fit wraps as for the bit-field itself.
struct F { unsigned bits : 3; int other; };
int main() {
  F f{1, 2};
  auto& [b, o] = f;
  b = 9;
  return f.bits == 1 && b == 1 ? 0 : 1;
}
