// tidal-rules: EXS-6
// tidal-expect: reject
// tidal-expect-gcc: accept
// tidal-expect-edg: accept
// tidal-note: Q-0079. Clang rejects a noexcept operand that depends on the function itself. GCC and EDG accept it.
struct S { void f() noexcept(noexcept(f())); };
bool b = noexcept(S{}.f());
