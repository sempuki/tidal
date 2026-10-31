// tidal-rules: VIRT-15
// tidal-expect: reject
struct X {};
struct Y {};
struct B { virtual X* get(); };
struct D : B { Y* get(); };
