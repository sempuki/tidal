// tidal-rules: PRIM-31
// tidal-expect: run
// tidal-note: Writing a number as text with '0' + digit, and reading it back with c - '0'.
int format(unsigned value, char* out) {
  char reversed[12];
  int n = 0;
  do {
    reversed[n++] = static_cast<char>('0' + value % 10);
    value /= 10;
  } while (value != 0);
  for (int i = 0; i < n; ++i) out[i] = reversed[n - 1 - i];
  out[n] = 0;
  return n;
}
int main() {
  char text[12];
  int n = format(305419896u, text);
  if (n != 9) return 1;
  unsigned back = 0;
  for (int i = 0; i < n; ++i) back = back * 10 + (text[i] - '0');
  return back == 305419896u && text[0] == '3' && text[8] == '6' ? 0 : 2;
}
