// tidal-rules: INST-20
// tidal-expect: accept
// tidal-expect-gcc: reject
// tidal-error-gcc: but no definition available
// tidal-note: The definition of the variable template follows the explicit instantiation definition in the same unit. Clang and EDG accept the program, GCC rejects it (Q-0254).
template <class T> extern int value;
template int value<int>;
template <class T> int value = 2;
