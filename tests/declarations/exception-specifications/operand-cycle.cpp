// tidal-rules: EXS-6
// tidal-expect: accept
// tidal-expect-clang: reject
// tidal-error-clang: exception specification is not available until end of class
// tidal-note: Q-0079. EXS-6 needs no diagnostic, so an implementation can accept or reject. GCC and EDG accept, as an implementation that does not detect the dependency does. Clang rejects.
struct S { void f() noexcept(noexcept(f())); };
bool b = noexcept(S{}.f());
