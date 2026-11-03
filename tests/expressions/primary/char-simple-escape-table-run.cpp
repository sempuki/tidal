// tidal-rules: PRIM-34
// tidal-expect: run
// tidal-note: The PRIM-34 values at run time, read from a string literal through a loop.
int main() {
  const char escapes[] = "\'\"\?\\\a\b\f\n\r\t\v";
  const int values[] = {0x27, 0x22, 0x3F, 0x5C, 0x07, 0x08, 0x0C, 0x0A, 0x0D, 0x09, 0x0B};
  if (sizeof escapes != 12) return 1;
  for (int i = 0; i < 11; ++i) {
    if (escapes[i] != values[i]) return 2;
  }
  return 0;
}
