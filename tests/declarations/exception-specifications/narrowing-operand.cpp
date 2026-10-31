// tidal-rules: EXS-1
// tidal-expect: reject
// tidal-expect-edg: accept
// tidal-note: Q-0078. EDG converts the operand to bool and permits narrowing. Clang and GCC reject it.
void f() noexcept(2);
