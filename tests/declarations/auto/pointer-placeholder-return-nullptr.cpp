// tidal-rules: AUTO-19 RET-1
// tidal-expect: reject
// tidal-error-gcc: nullptr
// tidal-error-clang: nullptr_t
// tidal-note: Write suite W18, finding W18-12 (reports/clean-room/W18/report.md). nullptr does not have pointer type, so auto* cannot deduce from return nullptr.
int x = 1;
auto* get(bool c) {
  if (c) return &x;
  return nullptr;
}
