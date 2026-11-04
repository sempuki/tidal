// tidal-rules: VIRT-3 VIRT-7
// tidal-expect: reject
// tidal-error-clang: overrides a 'final' function
// tidal-error-gcc: overriding final function
// tidal-error-edg: cannot override "final" function
// tidal-note: Derived declares no destructor. Its implicit destructor overrides the final destructor of Base.
struct Base { virtual ~Base() final; };
struct Derived : Base { };
