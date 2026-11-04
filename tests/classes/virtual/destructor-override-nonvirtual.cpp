// tidal-rules: VIRT-3 VIRT-7
// tidal-expect: reject
// tidal-error-clang: only virtual member functions can be marked 'override'
// tidal-error-gcc: marked .override., but does not override
// tidal-error-edg: declared with "override" does not
// tidal-note: The destructor of Base is not virtual, so the destructor of Derived overrides nothing.
struct Base { ~Base(); };
struct Derived : Base { ~Derived() override; };
