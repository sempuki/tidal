// tidal-rules: PFX-15
// tidal-expect: accept
// tidal-expect-clang: reject
// tidal-error-clang: cannot pass object of non-trivial type .D. through variadic function
// tidal-note: Conditionally supported. The class has only a non-trivial destructor. GCC and EDG support it. Clang rejects it.
struct D { ~D() {} };
void v(int, ...);
void f() { D d; v(0, d); }
