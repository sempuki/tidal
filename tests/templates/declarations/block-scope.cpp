// tidal-rules: TPL-2
// tidal-expect: reject
// tidal-error-clang: templates can only be declared in namespace or class scope
// tidal-error-clang-18: expected expression
// tidal-error-gcc: a template declaration cannot appear at block scope
// tidal-error-edg: a template declaration is not allowed here
// tidal-note: A class template declared in a function body.
int use() {
  template <class T> struct Local { T v; };
  return 0;
}
