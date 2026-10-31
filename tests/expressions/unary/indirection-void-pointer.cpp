// tidal-rules: UN-6
// tidal-expect: reject
// tidal-error-clang: indirection not permitted on operand of type .void \*.
// tidal-error-gcc: is not a pointer-to-object type
// tidal-error-edg: pointer to a complete object type
void f(void* p) {
  *p;
}
