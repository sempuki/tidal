// tidal-rules: STMT-8
// tidal-expect: reject
// tidal-error-clang: typedef redefinition with different types
// tidal-error-gcc: conflicting declaration .typedef long int Count
// tidal-error-edg: invalid redeclaration of type name .Count
void f() {
  typedef int Count;
  typedef long Count;
}
