// tidal-rules: JMP-14
// tidal-expect: reject
// tidal-expect-gcc: accept
// tidal-error-clang: undeclared label .nowhere
// tidal-error-edg: label .nowhere. was referenced but not defined
// tidal-note: X24-3. The label of a goto is checked at the definition of its function, also where no specialization is made or the statement is discarded. Clang and EDG reject. GCC accepts (Q-0451).
template <class T> void step() { if constexpr (sizeof(T) == 0) { goto nowhere; } }
void use() { step<int>(); }
