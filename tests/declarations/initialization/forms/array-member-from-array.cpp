// tidal-rules: INIT-8
// tidal-expect: reject
// tidal-error-clang: cannot initialize an array element of type .int. with an lvalue of type .const int\[2\].
// tidal-error-gcc: array used as initializer
// tidal-error-edg: cannot be used to initialize an entity of type .int.
// tidal-note: A member initializer gives an array member another array.
struct S {
  int a[2];
  S(const int (&x)[2]) : a(x) {}
};
