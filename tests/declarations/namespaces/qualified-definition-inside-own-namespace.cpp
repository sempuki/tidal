// tidal-rules: NS-15
// tidal-expect: reject
// tidal-expect-clang: accept
// tidal-error-gcc: explicit qualification in declaration
// tidal-error-edg: qualif
// tidal-note: Q-0371. GCC and EDG reject the qualified name inside its own namespace. Clang accepts it with a warning.
namespace geo {
  int area();
  int geo::area() { return 0; }   // qualified, inside geo itself
}
