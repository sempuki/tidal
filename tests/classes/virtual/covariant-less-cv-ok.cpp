// tidal-rules: VIRT-4
// tidal-expect: accept
struct B { virtual const B* get(); };
struct D : B { D* get() override; };
