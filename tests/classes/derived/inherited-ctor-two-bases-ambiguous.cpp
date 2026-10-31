// tidal-rules: DER-15
// tidal-expect: reject
// tidal-error-clang: call to constructor of .D. is ambiguous
// tidal-error-gcc: call of overloaded .D\(int\). is ambiguous
// tidal-error-edg: more than one instance of constructor .D::D
struct A { A() {} A(int) {} };
struct B { B() {} B(int) {} };
struct D : A, B { using A::A; using B::B; };
D d(1);
