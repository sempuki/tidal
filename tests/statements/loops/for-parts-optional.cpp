// tidal-rules: LOOP-3
// tidal-expect: run
int main() {
  int i = 0;
  for (;;) { if (++i == 3) break; }
  for (; i < 5;) ++i;
  return i == 5 ? 0 : 1;
}
