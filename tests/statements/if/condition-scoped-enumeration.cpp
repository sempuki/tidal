// tidal-rules: IF-4
// tidal-expect: reject
// tidal-error-clang: not contextually convertible to .bool
// tidal-error-gcc: could not convert .* to .bool
// tidal-error-edg: must have bool type
// tidal-note: A scoped enumeration has no contextual conversion to bool.
enum class Mode { off, on };
void f(Mode m) {
  if (m) {}
}
