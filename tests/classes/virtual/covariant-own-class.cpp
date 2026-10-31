// tidal-rules: VIRT-4
// tidal-expect: accept
struct B { virtual B* clone(); };
struct D : B { D* clone() override; };
