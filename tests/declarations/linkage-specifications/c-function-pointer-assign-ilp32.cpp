// tidal-rules: LSP-6
// tidal-expect: accept
// tidal-flags: -m32
// tidal-expect-edg: reject
// tidal-error-edg: cannot be used to initialize an entity
// tidal-note: Q-0074. -m32. EDG treats C language linkage as part of the function type. Clang and GCC do not.
extern "C" void cfun() {}
void (*p)() = cfun;
