// tidal-rules: VIRT-15
// tidal-expect: reject
struct A {};
struct B : A {};
struct C : A {};
struct R : B, C {};
struct Base { virtual A* get(); };
struct D : Base { R* get(); };
