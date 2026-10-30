// tidal-rules: LOOP-1
// tidal-expect: run
int main() {
  int n = 0, i = 0;
  while (i < 3) { n += i; ++i; }
  int never = 0;
  while (false) never = 1;
  return n == 3 && never == 0 ? 0 : 1;
}
