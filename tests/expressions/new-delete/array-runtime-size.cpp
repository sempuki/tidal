// tidal-rules: NEW-2
// tidal-expect: run
int main(int argc, char**) {
  int n = argc + 2;
  int* a = new int[n];
  for (int i = 0; i < n; ++i) a[i] = i;
  int last = a[n - 1];
  delete[] a;
  return last == n - 1 ? 0 : 1;
}
