// tidal-rules: VIRT-2
// tidal-expect: reject
// tidal-error-clang: explicit object parameter cannot appear in a virtual function
// tidal-error-gcc: explicit object member function cannot be .virtual
// tidal-error-edg: virtual member function cannot have an explicit .this. parameter
struct S { virtual void f(this S&); };
