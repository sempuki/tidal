// tidal-rules: ATTR-2
// tidal-expect: reject
// tidal-expect-clang: accept
// tidal-expect-gcc: accept
// tidal-error-edg: attribute .nodiscard. does not apply here
// tidal-note: RV11-R-3. Before a using-directive, the attribute applies to the directive, and nodiscard does not apply to a directive. EDG rejects. Clang and GCC accept (Q-0021).
namespace tools { int count = 2; }
[[nodiscard]] using namespace tools;
