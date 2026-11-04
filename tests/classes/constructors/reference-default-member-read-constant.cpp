// tidal-rules: CTOR-17 CE-9
// tidal-expect: reject
// tidal-error-clang: binds to a temporary object whose lifetime
// tidal-error-gcc: outside its lifetime
// tidal-error-edg: expired storage
// tidal-note: Q-0084. Constant evaluation reads the reference member after its temporary ended. Clang rejects the binding. GCC and EDG reject the read.
struct S { const int& r = 42; };
constexpr int read() { S s; return s.r; }
static_assert(read() == 42);
