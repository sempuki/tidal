// tidal-rules: VIRT-3
// tidal-expect: reject
// tidal-error-clang: already marked 'override'
// tidal-error-gcc: duplicate virt-specifier
// tidal-error-edg: duplicate function modifier
struct Base { virtual void run(); };
struct Derived : Base { void run() override override; };
