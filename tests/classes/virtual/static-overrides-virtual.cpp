// tidal-rules: VIRT-2
// tidal-expect: reject
// tidal-error-clang: static. member function .f. overrides a virtual function
// tidal-error-gcc: declared in base class
// tidal-error-edg: only nonstatic member functions may be virtual
struct B { virtual void f(); };
struct D : B { static void f(); };
