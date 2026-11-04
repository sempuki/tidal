// tidal-rules: ALN-2
// tidal-expect: reject
// tidal-expect-gcc-14: accept
// tidal-error-clang: cannot be applied to a 'catch' variable
// tidal-error-gcc: alignas. on exception declaration
// tidal-error-edg: attribute
// tidal-note: The variable that a handler declares cannot have alignas. GCC 14 accepts the declaration, and GCC 16 rejects it.
void run() {
  try {
  } catch (alignas(16) int code) {
  }
}
