// tidal-rules: JMP-14
// tidal-expect: reject
// tidal-error-clang: undeclared label .out
// tidal-error-gcc: label .out. used but not defined
// tidal-error-edg: label .out. was referenced but not defined
// tidal-note: Gap review 2026-10-31 (third).
void f() { out: ; auto l = [] { goto out; }; l(); }
