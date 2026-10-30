// tidal-rules: RFOR-3
// tidal-expect: run
int main() {
  int sum = 0;
  for (int a[] = {1, 2}; int v : a) sum += v;
  return sum == 3 ? 0 : 1;
}
