// tidal-rules: VIRT-2
// tidal-expect: reject
// tidal-error-clang: unions cannot have virtual functions
// tidal-error-gcc: declared .virtual. inside a union
// tidal-error-edg: virtual. is not allowed
union U { virtual void f(); int i; };
