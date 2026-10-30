// tidal-rules: RFOR-1
// tidal-expect: run
int main() {
  int a[] = {1, 2, 3};
  int sum = 0;
  for (int v : a) sum += v;
  for (int& v : a) v *= 2;
  return sum == 6 && a[2] == 6 ? 0 : 1;
}
