// tidal-rules: VIRT-13
// tidal-expect: accept
// tidal-flags: -Wall -Werror
// tidal-note: G165. D::f(int) overrides B::f(int) and hides B::f(double). The warning of VIRT-13 concerns a derived name none of whose functions overrides, so -Wall gives no warning here on GCC or Clang. Under -Woverloaded-virtual alone, GCC also warns for this case.
struct B { virtual void f(int); virtual void f(double); virtual ~B(); };
struct D : B { void f(int) override; };
