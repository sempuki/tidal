// tidal-rules: ATTR-2
// tidal-expect: reject
// tidal-expect-clang: accept
// tidal-expect-gcc: accept
// tidal-error-edg: attribute .deprecated. does not apply here
// tidal-note: RV11-R-3. Before a using-directive, the attribute applies to the directive, and deprecated does not apply to a directive. EDG rejects. Clang and GCC accept (Q-0021).
namespace tools { int count = 2; }
[[deprecated]] using namespace tools;
