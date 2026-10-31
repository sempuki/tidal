// tidal-rules: ADD-3
// tidal-expect: reject
// tidal-error-clang: invalid operands to binary expression .*nullptr_t
// tidal-error-gcc: invalid operands of types .std::nullptr_t. and .int.
// tidal-error-edg: nullptr_t
// tidal-note: Gap review 2026-10-31 (third).
auto p = nullptr + 1;
