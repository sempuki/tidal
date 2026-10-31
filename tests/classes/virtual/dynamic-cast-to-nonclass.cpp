// tidal-rules: VIRT-10
// tidal-expect: reject
struct B { virtual ~B() {} };
int* f(B* p) { return dynamic_cast<int*>(p); }
