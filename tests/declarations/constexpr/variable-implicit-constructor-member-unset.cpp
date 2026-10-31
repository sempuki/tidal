// tidal-rules: CX-1 CTOR-4
// tidal-expect: reject
// tidal-error-clang: default initialization of an object of const type
// tidal-error-gcc: uninitialized .const settings.
// tidal-error-edg: requires an initializer
struct Settings {
  int width = 80;
  int height;
};
constexpr Settings settings;
