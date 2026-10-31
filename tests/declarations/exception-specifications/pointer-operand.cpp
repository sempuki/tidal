// tidal-rules: EXS-1
// tidal-expect: reject
// tidal-expect-edg: accept
// tidal-note: Q-0078. EDG converts the operand to bool and permits narrowing. Clang and GCC reject it.
constexpr int x = 0;
void f() noexcept(&x);
