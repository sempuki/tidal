// tidal-rules: STG-11 STG-13 STG-26 TYP-1 UN-12
// tidal-expect: run
// tidal-note: Roadmap round 4, programmer sketch bits.cpp (reports/clean-room/roadmap-4/programmer.md, section 1.5). The rules are the ones the report names for this program and its reductions, and the rules of the main constructs it uses.
// tidal-note: A program test is evidence from use and decides no rule alone (decision 0054).
// Read a float's bits at run time through unsigned char (no bit cast).
int main() {
  float f = 1.0f; unsigned u = 0;
  static_assert(sizeof f == sizeof u);
  const unsigned char* src = reinterpret_cast<const unsigned char*>(&f);
  unsigned char* dst = reinterpret_cast<unsigned char*>(&u);
  for (unsigned i = 0; i < sizeof f; ++i) dst[i] = src[i];
  return u == 0x3f800000u ? 0 : 1;
}
