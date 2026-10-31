// tidal-rules: STO-6
// tidal-expect: reject
// tidal-error-clang: function declared in block scope cannot have .static. storage class
// tidal-error-gcc: cannot declare static function inside another function|specifier invalid for function .g. declared out of global scope
// tidal-error-edg: block-scope function may only have extern storage class
void f() {
  static int g();
}
