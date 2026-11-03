// tidal-rules: EXS-6
// tidal-expect: accept
// tidal-expect-clang: reject
// tidal-error-clang: exception specification is not available until end of class
// tidal-note: Q-0079. A static member function whose noexcept operand calls the function itself. EXS-6 needs no diagnostic, so an implementation can accept or reject. GCC and EDG accept. Clang rejects.
struct S { static void g() noexcept(noexcept(g())); };
bool b = noexcept(S::g());
