// tidal-rules: MUL-17
// tidal-expect: accept
// tidal-flags: -m32
// tidal-note: GCC keeps x87 precision in floating-point intermediates on 32-bit x86. A cast, an initializer, and a return value round to the type on every implementation.
constexpr double tiny = 0x1p-53;
constexpr float tiny_f = 0x1p-24f;
static_assert(static_cast<double>(1.0 + tiny) == 1.0);
static_assert(static_cast<float>(1.0f + tiny_f) == 1.0f);
constexpr double sum = 1.0 + tiny;
static_assert(sum == 1.0);
constexpr double add(double a, double b) { return a + b; }
static_assert(add(1.0, tiny) == 1.0);
