// tidal-rules: VIRT-13
// tidal-expect: accept
// tidal-flags: -Wsuggest-override -Werror
// tidal-note: Every override in Derived carries override or final, so no warning applies.
struct Base { virtual void start(); virtual void stop(); };
struct Derived : Base { void start() override; void stop() final; };
