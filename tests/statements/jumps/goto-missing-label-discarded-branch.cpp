// tidal-rules: JMP-14
// tidal-expect: reject
// tidal-expect-gcc: accept
// tidal-error-clang: undeclared label .nowhere
// tidal-error-edg: label .nowhere. was referenced but not defined
// tidal-note: X24-3. A goto names a missing label in a discarded statement of a function template that the program uses. JMP-14 rejects the program, as outside a template. Clang and EDG reject, and GCC accepts (Q-0451).
template <class T> void step() { if constexpr (sizeof(T) == 0) { goto nowhere; } }
void use() { step<int>(); }
