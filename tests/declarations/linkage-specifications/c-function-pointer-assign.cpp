// tidal-rules: LSP-6
// tidal-expect: accept
// tidal-expect-edg: reject
// tidal-note: Q-0074. EDG treats C language linkage as part of the function type. Clang and GCC do not.
extern "C" void cfun() {}
void (*p)() = cfun;
