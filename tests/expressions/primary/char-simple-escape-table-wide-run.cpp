// tidal-rules: PRIM-34
// tidal-expect: run
// tidal-note: The PRIM-34 values at run time, in the wide encoding.
int main() {
  const wchar_t escapes[] = L"\'\"\?\\\a\b\f\n\r\t\v";
  const int values[] = {0x27, 0x22, 0x3F, 0x5C, 0x07, 0x08, 0x0C, 0x0A, 0x0D, 0x09, 0x0B};
  for (int i = 0; i < 11; ++i) {
    if (escapes[i] != values[i]) return 1;
  }
  return escapes[11] == 0 ? 0 : 2;
}
