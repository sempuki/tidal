// tidal-rules: LOOP-4
// tidal-expect: run
int main() {
  int sum = 0;
  for (int i = 0; int left = 3 - i; ++i) sum += left;
  return sum == 6 ? 0 : 1;
}
