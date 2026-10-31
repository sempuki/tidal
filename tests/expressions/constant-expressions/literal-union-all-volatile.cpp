// tidal-rules: CE-23 CX-1
// tidal-expect: reject
// tidal-error-clang: of volatile type
// tidal-error-gcc: has volatile type
// tidal-error-edg: must have a literal type
union Register {
  volatile int hardware;
  volatile long wide;
  constexpr Register() : hardware(0) {}
};
constexpr Register reg;
