// tidal-rules: JMP-7 DEP-30
// tidal-expect: reject
// tidal-error-clang: continue. statement not in loop
// tidal-error-gcc: continue statement not within a loop
// tidal-error-edg: continue statement may only be used within a loop
// tidal-note: X24-3, RV10-14. A continue stands outside a loop in a function template that no specialization uses. Every compiler rejects the program, and DEP-30 allows that result.
template <class T> void step() { continue; }
