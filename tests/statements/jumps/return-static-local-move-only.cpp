// tidal-rules: JMP-15
// tidal-expect: reject
// tidal-error-clang: implicitly-deleted copy constructor of .M
// tidal-error-gcc: use of deleted function .*M::M\(const M&\)
// tidal-error-edg: M::M\(const M &\).*deleted function
// tidal-note: Gap review 2026-10-31 (third).
struct M { M() {} M(M&&) {} };
M f() { static M m; return m; }
