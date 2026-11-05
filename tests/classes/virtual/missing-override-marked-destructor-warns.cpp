// tidal-rules: VIRT-13 VIRT-7
// tidal-expect: reject
// tidal-flags: -Wsuggest-override -Werror
// tidal-error-clang: overrides a member function but is not marked 'override'
// tidal-error-gcc: can be marked override
// tidal-note: Derived marks its destructor with override and leaves start unmarked. The marked destructor counts as the other override, so the compilers warn about start.
struct Base { virtual ~Base(); virtual void start(); };
struct Derived : Base { ~Derived() override; void start(); };
