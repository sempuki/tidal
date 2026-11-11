// tidal-rules: ATTR-2
// tidal-expect: reject
// tidal-expect-gcc: accept
// tidal-error-clang: maybe_unused. attribute cannot be applied to types
// tidal-error-edg: attribute .maybe_unused. does not apply here
// tidal-note: Roadmap round 14 writer probe. After the type specifiers, the attribute applies to the type int, and maybe_unused does not apply to a type. Clang and EDG reject. GCC accepts (Q-0021).
int [[maybe_unused]] c;
