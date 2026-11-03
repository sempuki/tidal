// tidal-rules: PRIM-31
// tidal-expect: run
// tidal-note: A run-time parser that takes each digit's value as c - '0'. The text comes from an array the program changes, so the parse happens when the program runs.
long parse(const char* s) {
  long value = 0;
  for (; *s != 0; ++s) value = value * 10 + (*s - '0');
  return value;
}
int main() {
  char text[] = "4096";
  if (parse(text) != 4096) return 1;
  text[0] = '7';
  text[3] = '1';
  return parse(text) == 7091 ? 0 : 2;
}
