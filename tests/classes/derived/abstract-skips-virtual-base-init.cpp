// tidal-rules: DER-13
// tidal-expect: accept
struct V { V(int) {} };
struct A : virtual V { A() {} virtual void f() = 0; };
struct C : A { C() : V(1) {} void f() override {} };
C c;
