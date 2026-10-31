// tidal-rules: VIRT-13
// tidal-expect: reject
// tidal-error-clang: virtual functions but non-virtual destructor
// tidal-error-gcc: which has non-virtual destructor
// tidal-flags: -Wall -Werror
struct B { virtual void f(); };
struct D : B {};
void drop(B* p) { delete p; }
