// tidal-rules: JMP-14
// tidal-expect: reject
// tidal-error-clang: redefinition of label .again
// tidal-error-gcc: duplicate label .again
// tidal-error-edg: label .again. has already been defined
// tidal-note: X24-3. A label name that occurs twice is rejected at the definition of a function template that no specialization uses.
template <class T> void step() {
again:;
again:;
}
