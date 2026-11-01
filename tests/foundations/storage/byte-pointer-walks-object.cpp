// tidal-rules: STG-13 ADD-6
// tidal-expect: run
// tidal-flags: -O0 -fsanitize=address,undefined -fno-sanitize-recover=all
int main() {
  long value = 0x0102030405060708;
  const unsigned char* p = reinterpret_cast<const unsigned char*>(&value);
  const unsigned char* end = p + sizeof value;   // one past the last byte
  int sum = 0;
  for (; p != end; ++p) sum += *p;
  return sum == 36 ? 0 : 1;
}
