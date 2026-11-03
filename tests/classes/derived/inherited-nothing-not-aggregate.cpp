// tidal-rules: DER-8 CTOR-25
// tidal-expect: reject
// tidal-note: D inherits no constructor from B, but the using-declaration makes D a class that is not an aggregate. The same braces initialize a class without the using-declaration.
// tidal-error-clang: no matching constructor for initialization of .D
// tidal-error-gcc: no matching function for call to .D::D\(<brace-enclosed initializer list>\)
// tidal-error-edg: no instance of constructor .D::D
struct B { };
struct D : B { using B::B; int x; };
D d{{}, 1};
