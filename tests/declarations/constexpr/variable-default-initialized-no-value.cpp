// tidal-rules: CX-1
// tidal-expect: reject
// tidal-error-clang: default initialization of an object of const type
// tidal-error-gcc: uninitialized .const p
// tidal-error-edg: const variable "p" requires an initializer
struct Point { int x; };
constexpr Point p;
