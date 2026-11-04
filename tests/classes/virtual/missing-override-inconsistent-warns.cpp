// tidal-rules: VIRT-13
// tidal-expect: reject
// tidal-error-clang: overrides a member function but is not marked 'override'
// tidal-error-gcc: can be marked override
// tidal-flags: -Wsuggest-override -Werror
// tidal-note: Derived marks start with override and leaves stop unmarked.
struct Base { virtual void start(); virtual void stop(); };
struct Derived : Base { void start() override; void stop(); };
