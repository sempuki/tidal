union Bits {
  float f;
  unsigned u;
};
int main() {
  Bits b;
  b.f = 1.0f;
  return b.u == 0x3f800000u ? 0 : 1;  // reads the member that is not active
}
