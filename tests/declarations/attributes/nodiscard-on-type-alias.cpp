// tidal-rules: ATTR-2
// tidal-expect: reject
// tidal-expect-clang: accept
// tidal-expect-gcc: accept
// tidal-error-edg: attribute .nodiscard. does not apply here
// tidal-note: Q-0021. nodiscard goes on a function, a class, or an enumeration, and a type alias is none of these. Clang and GCC ignore the attribute with a warning. EDG rejects it.
using Status [[nodiscard]] = int;
