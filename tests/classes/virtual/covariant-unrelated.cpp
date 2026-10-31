// tidal-rules: VIRT-4
// tidal-expect: reject
struct X {};
struct Y {};
struct B { virtual X* get(); };
struct D : B { Y* get(); };
