// tidal-rules: ODR-15
// tidal-expect: accept
// tidal-note: A static data member declared constexpr in its class is defined there. A declaration of it outside the class without an initializer is not a definition, so it can appear twice. The form is deprecated.
struct Limits { static constexpr int top = 9; };
constexpr int Limits::top;
constexpr int Limits::top;
const int* where = &Limits::top;
