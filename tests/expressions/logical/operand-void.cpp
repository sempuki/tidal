// tidal-rules: LOG-3
// tidal-expect: reject
// tidal-error-clang: value of type .void. is not contextually convertible
// tidal-error-gcc: from .void. to .bool.
// tidal-error-edg: must have bool type
void g();
void f() {
  bool b = g() && true;
}
