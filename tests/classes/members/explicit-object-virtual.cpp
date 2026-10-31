// tidal-rules: CLS-14
// tidal-expect: reject
// tidal-error-clang: explicit object parameter cannot appear in a virtual function
// tidal-error-gcc: explicit object member function cannot be .virtual
// tidal-error-edg: virtual member function cannot have an explicit .this. parameter
// tidal-note: Gap review 2026-10-31.
struct S { virtual void f(this S&); };
