// tidal-rules: EXS-7
// tidal-expect: reject
// tidal-expect-edg: accept
// tidal-error-clang: cannot be narrowed to type .bool
// tidal-error-gcc: narrowing conversion
// tidal-note: Q-0078. EDG converts the operand to bool and permits narrowing. Clang and GCC reject it.
void f() noexcept(2);
