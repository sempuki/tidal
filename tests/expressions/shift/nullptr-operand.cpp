// tidal-rules: SHF-3
// tidal-expect: reject
// tidal-error-clang: invalid operands to binary expression .*nullptr_t
// tidal-error-gcc: invalid operands of types .std::nullptr_t.
// tidal-error-edg: must have integral or unscoped enum type
// tidal-note: Gap review 2026-10-31 (third).
auto x = nullptr << 1;
