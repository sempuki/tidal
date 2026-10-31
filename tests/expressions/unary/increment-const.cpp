// tidal-rules: UN-11
// tidal-expect: reject
// tidal-error-clang: const-qualified type
// tidal-error-gcc: increment of read-only variable
// tidal-error-edg: must be a modifiable lvalue
void f() {
  const int c = 0;
  ++c;
}
