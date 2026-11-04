// tidal-rules: VOL-6
// tidal-expect: accept
// tidal-note: Q-0359. The loop reads the bytes of the volatile member through unsigned char glvalues that are not volatile, which VOL-6 makes undefined. No sanitizer reports the read, and constant evaluation cannot form the byte pointers, so the test only builds the program.
struct Port { volatile int status; int count; };
int main() {
  Port a{7, 9}, b{0, 0};
  unsigned char* to = reinterpret_cast<unsigned char*>(&b);
  const unsigned char* from = reinterpret_cast<const unsigned char*>(&a);
  for (unsigned i = 0; i < sizeof(Port); ++i) to[i] = from[i];
  return b.count == 9 ? 0 : 1;
}
