// tidal-rules: CV-3
// tidal-expect: reject
// tidal-error-clang: default initialization of an object of const type
// tidal-error-gcc: uninitialized .const x
// tidal-error-edg: requires an initializer
const int x;
