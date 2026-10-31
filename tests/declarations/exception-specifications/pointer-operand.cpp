// tidal-rules: EXS-7
// tidal-expect: reject
// tidal-expect-edg: accept
// tidal-error-clang: to .bool. is not allowed in a converted constant
// tidal-error-gcc: to .bool. in a converted constant
// tidal-note: Q-0078. EDG converts the operand to bool and permits narrowing. Clang and GCC reject it.
constexpr int x = 0;
void f() noexcept(&x);
