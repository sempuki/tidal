// tidal-rules: CX-1
// tidal-expect: reject
// tidal-error-clang: constexpr variable .n. must be initialized
// tidal-error-gcc: uninitialized .const n
// tidal-error-edg: missing initializer for constexpr variable
constexpr int n;
