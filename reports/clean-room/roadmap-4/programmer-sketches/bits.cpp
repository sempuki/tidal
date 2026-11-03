// Read a float's bits at run time through unsigned char (no bit cast).
int main() {
  float f = 1.0f; unsigned u = 0;
  static_assert(sizeof f == sizeof u);
  const unsigned char* src = reinterpret_cast<const unsigned char*>(&f);
  unsigned char* dst = reinterpret_cast<unsigned char*>(&u);
  for (unsigned i = 0; i < sizeof f; ++i) dst[i] = src[i];
  return u == 0x3f800000u ? 0 : 1;
}
