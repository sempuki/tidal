// tidal-rules: CV-4
// tidal-expect: reject
// tidal-error-clang: read-only variable is not assignable
// tidal-error-gcc: assignment of read-only location
// tidal-error-edg: must be a modifiable lvalue
void f(const int* p) {
  *p = 1;
}
