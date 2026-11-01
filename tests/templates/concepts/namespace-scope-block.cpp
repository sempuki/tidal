// tidal-rules: CPT-4
// tidal-expect: reject
// tidal-error-clang: may only appear in global or namespace scope|expected expression
// tidal-error-gcc: cannot appear at block scope
// tidal-error-edg: template declaration is not allowed
void run() {
  template <class T> concept Inner = true;
}
