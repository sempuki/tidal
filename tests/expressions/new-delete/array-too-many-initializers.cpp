// tidal-rules: NEW-2
// tidal-expect: reject
void f() {
  int* a = new int[2]{1, 2, 3};
  delete[] a;
}
