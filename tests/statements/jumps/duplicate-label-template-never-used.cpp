// tidal-rules: JMP-14 DEP-30
// tidal-expect: reject
// tidal-error-clang: redefinition of label .again
// tidal-error-gcc: duplicate label .again
// tidal-error-edg: label .again. has already been defined
// tidal-note: X24-3, RV10-14. A label name occurs twice in a function template that no specialization uses. Every compiler rejects the program, and DEP-30 allows that result.
template <class T> void step() {
again:;
again:;
}
