// tidal-rules: BF-4 VOL-10
// tidal-expect: reject
// tidal-error-clang: cannot bind to bit-field
// tidal-error-gcc: cannot bind bit-field
// tidal-error-edg: bit field
struct Flags { int mode : 3; };
Flags flags;
const volatile int& r = flags.mode;   // no temporary for a volatile reference
