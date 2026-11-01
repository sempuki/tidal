// tidal-rules: ORD-9
// tidal-expect: accept
// tidal-note: With a target type, the whole function type takes part, also the result type. T* make(int) is more specialized than T make(int).
int slot = 0;
template <class T> constexpr T* make(int) { return nullptr; }
template <class T> constexpr T make(int) { return &slot; }
constexpr int* (*p)(int) = make;
static_assert(p(0) == nullptr);
template <class T> constexpr int f(T) { return 1; }
template <class T> constexpr int f(T*) { return 2; }
constexpr int (*q)(int*) = f;
static_assert(q(nullptr) == 2);
