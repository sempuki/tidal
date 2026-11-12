// tidal-rules: ATTR-2
// tidal-expect: reject
// tidal-expect-clang: accept
// tidal-expect-gcc: accept
// tidal-error-edg: attribute .maybe_unused. does not apply here
// tidal-note: RV10-3 probe. Before a using-directive, the attribute applies to the directive, and maybe_unused does not apply to a directive. EDG rejects. Clang and GCC accept (Q-0021).
namespace tools { int count = 2; }
[[maybe_unused]] using namespace tools;
