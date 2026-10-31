// tidal-rules: VIRT-19
// tidal-expect: reject
// tidal-expect-edg: accept
// tidal-note: Q-0087. EDG accepts an explicit object member function with the name and parameters of a base virtual function. Clang and GCC reject it.
struct B { virtual void f(); };
struct D : B { void f(this D&); };
