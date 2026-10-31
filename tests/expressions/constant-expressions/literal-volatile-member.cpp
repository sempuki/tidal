// tidal-rules: CE-23 CX-1
// tidal-expect: reject
// tidal-error-clang: data member .level. of volatile type
// tidal-error-gcc: .Port::level. has volatile type
// tidal-error-edg: must have a literal type
struct Port {
  volatile int level;
  constexpr Port() : level(0) {}
};
constexpr Port port;
