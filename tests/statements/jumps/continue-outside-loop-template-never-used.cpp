// tidal-rules: JMP-7
// tidal-expect: reject
// tidal-error-clang: continue. statement not in loop
// tidal-error-gcc: continue statement not within a loop
// tidal-error-edg: continue statement may only be used within a loop
// tidal-note: X24-3. A continue outside a loop is rejected at the definition of a function template that no specialization uses.
template <class T> void step() { continue; }
