// tidal-rules: NS-1
// tidal-expect: reject
// tidal-error-clang: namespaces can only be defined in global or namespace scope
// tidal-error-gcc: namespace. definition is not allowed here
// tidal-error-edg: namespace definition is not allowed
void f() {
  namespace N {}
}
