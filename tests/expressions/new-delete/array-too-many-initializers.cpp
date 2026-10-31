// tidal-rules: NEW-2
// tidal-expect: reject
// tidal-error-clang: excess elements in array initializer
// tidal-error-gcc: too many initializers
// tidal-error-edg: too many initializer values
void f() {
  int* a = new int[2]{1, 2, 3};
  delete[] a;
}
