// tidal-rules: DCL-3
// tidal-expect: reject
// tidal-error-clang: reference variable .r. requires an initializer
// tidal-error-gcc: declared as reference but not initialized
// tidal-error-edg: reference variable .r. requires an initializer
void f() {
  int& r;
}
