// tidal-rules: ADD-6
// tidal-expect: run
int main() {
  int a[4] = {10, 20, 30, 40};
  int* p = a;
  p = p + 3;
  int* q = p - 2;
  return *p == 40 && *q == 20 && p - q == 2 ? 0 : 1;
}
