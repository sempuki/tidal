// tidal-rules: VIRT-13
// tidal-expect: reject
// tidal-error-clang: hides overloaded virtual function
// tidal-error-gcc: was hidden
// tidal-flags: -Wall -Werror
struct B { virtual void f(int); };
struct D : B { void f(const char*); };
