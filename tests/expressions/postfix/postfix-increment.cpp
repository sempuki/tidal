// tidal-rules: PFX-12
// tidal-expect: run
int main() {
  int i = 5;
  int old = i++;
  int j = 5;
  int old2 = j--;
  int a[3] = {};
  int* p = a;
  int* q = p++;
  return old == 5 && i == 6 && old2 == 5 && j == 4 && q == a && p == a + 1 ? 0 : 1;
}
