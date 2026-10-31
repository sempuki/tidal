// tidal-rules: VOL-2
// tidal-expect: reject
// tidal-error-clang: function is not marked volatile
// tidal-error-gcc: volatile Port. as .this. argument discards qualifiers
// tidal-error-edg: type qualifiers that are not compatible with the member function
struct Port { int v; int read() { return v; } };
int f(volatile Port& p) { return p.read(); }
