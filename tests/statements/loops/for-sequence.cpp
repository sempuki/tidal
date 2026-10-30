// tidal-rules: LOOP-3
// tidal-expect: run
int main() {
  int trace[8], n = 0;
  for (int i = (trace[n++] = 1, 0); (trace[n++] = 2, i < 2); trace[n++] = 4, ++i) trace[n++] = 3;
  int want[] = {1, 2, 3, 4, 2, 3, 4, 2};
  for (int k = 0; k < 8; ++k) if (trace[k] != want[k]) return 1;
  return n == 8 ? 0 : 2;
}
