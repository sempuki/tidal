// tidal-rules: ICS-44
// tidal-expect: accept
// tidal-expect-clang: reject
// tidal-error-clang: invalid suffix .f(32|64). on floating constant
// tidal-note: PRIM-22: Clang has no f32 or f64 suffix. Each result converts to the target by conversion rank. Only the source of equal rank decides.
template <class T, class U> constexpr bool same = false;
template <class T> constexpr bool same<T, T> = true;
struct One {};
struct Two {};
using Float32 = decltype(1.0f32);
using Float64 = decltype(1.0f64);
struct Single {
  constexpr operator Float32() const { return 1; }
  constexpr operator double() const { return 2; }
};
constexpr float f = Single{};
static_assert(f == 1);
struct Whole {
  constexpr operator Float32() const { return 1; }
  constexpr operator int() const { return 2; }
};
constexpr float g = Whole{};
static_assert(g == 1);
struct Wide {
  constexpr operator Float64() const { return 1; }
  constexpr operator long double() const { return 2; }
};
constexpr double h = Wide{};
static_assert(h == 1);
