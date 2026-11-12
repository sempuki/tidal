// tidal-rules: JMP-14 DEP-30
// tidal-expect: reject
// tidal-expect-gcc: accept
// tidal-error-clang: undeclared label .nowhere
// tidal-error-edg: label .nowhere. was referenced but not defined
// tidal-note: X24-3, RV10-14. A goto names a missing label in a function template that no specialization uses. The condition does not depend on the parameters, so DEP-30 lets an implementation accept the template with no diagnostic. Clang and EDG reject, and GCC accepts (Q-0451).
template <class T> void step() { goto nowhere; }
