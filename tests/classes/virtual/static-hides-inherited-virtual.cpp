// tidal-rules: VIRT-21
// tidal-expect: reject
// tidal-error-clang: static. member function .f. overrides a virtual function
// tidal-error-gcc: declared in base class
// tidal-error-edg: only nonstatic member functions may be virtual
// tidal-note: The virtual function has a parameter and comes from a base two levels up.
struct B { virtual int f(int); };
struct M : B {};
struct D : M { static int f(int); };
