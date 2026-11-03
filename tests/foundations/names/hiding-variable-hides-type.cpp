// tidal-rules: NAM-12
// tidal-expect: reject
// tidal-error-clang: must use .struct. tag to refer to type .T. in this scope
// tidal-error-gcc: expected .;. before .t.
// tidal-error-edg: expected a ";"
// tidal-note: The inner declaration is a variable, and the use needs a type. The hidden class would fit the use.
struct T {};
void f() {
  int T = 1;
  T t;
}
