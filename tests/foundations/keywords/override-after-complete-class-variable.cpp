// tidal-rules: KW-3
// tidal-expect: accept
// tidal-expect-gcc-14: reject
// tidal-error-gcc-14: cannot specify .override. for a class
// tidal-note: Q-0382. GCC 14 reads override as a marker of the class and rejects it. override after the name of a complete class declares a variable named override with a braced initializer.
struct D { int value; };
struct D override { 3 };
static_assert(sizeof(override) == sizeof(int));
int read() { return override.value; }
