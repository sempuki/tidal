// tidal-rules: VOL-9
// tidal-expect: reject
// tidal-error-clang: function is not marked volatile
// tidal-error-gcc: volatile Port. as .this. argument discards qualifiers
// tidal-error-edg: type qualifiers that are not compatible with the member function
// tidal-note: The member function is const but not volatile.
struct Port { int v; int read() const { return v; } };
int f(volatile Port& p) { return p.read(); }
