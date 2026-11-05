// tidal-rules: VIRT-13 VIRT-7
// tidal-expect: accept
// tidal-flags: -Wsuggest-override -Werror
// tidal-note: Derived marks start with override and leaves its destructor unmarked. The destructor overrides the virtual destructor of Base, and no compiler warns about a destructor without override.
struct Base { virtual ~Base(); virtual void start(); };
struct Derived : Base { ~Derived(); void start() override; };
