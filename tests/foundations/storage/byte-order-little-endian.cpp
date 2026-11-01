// tidal-rules: STG-26
// tidal-expect: run
int main() {
  unsigned value = 0x04030201;
  const unsigned char* bytes = reinterpret_cast<const unsigned char*>(&value);
  int negative = -1;
  const unsigned char* ones = reinterpret_cast<const unsigned char*>(&negative);
  return bytes[0] == 1 && bytes[1] == 2 && bytes[2] == 3 && bytes[3] == 4 && ones[0] == 0xFF ? 0 : 1;
}
