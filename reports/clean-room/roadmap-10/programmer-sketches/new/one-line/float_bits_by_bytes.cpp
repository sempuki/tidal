// The bits of a float read by copying its bytes into an unsigned int.
static_assert(sizeof(float) == sizeof(unsigned));
unsigned bits_of(float f) {
  unsigned u = 0;
  const unsigned char* src = reinterpret_cast<const unsigned char*>(&f);
  unsigned char* dst = reinterpret_cast<unsigned char*>(&u);
  for (unsigned i = 0; i < sizeof f; ++i) dst[i] = src[i];
  return u;
}
int main() { return bits_of(1.0f) == 0x3f800000u && bits_of(-2.0f) == 0xc0000000u ? 0 : 1; }
