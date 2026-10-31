// tidal-rules: VIRT-4
// tidal-expect: accept
struct B { virtual B& get(); };
struct D : B { D& get() override; };
