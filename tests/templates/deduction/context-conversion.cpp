// tidal-rules: DED-31
// tidal-expect: accept
// tidal-note: The conversion function template deduces T from the type that the initialization needs. A reference target gives its referred type, and top-level const goes.
template <class T> constexpr int code = 0;
template <> constexpr int code<int> = 1;
template <> constexpr int code<const int> = 2;
template <> constexpr int code<int*> = 3;
template <> constexpr int code<const int*> = 4;
struct Number {
  template <class T> constexpr operator T() const { return T(code<T>); }
};
struct Address {
  template <class T> constexpr operator T*() const { return code<T> == 2 ? nullptr : nullptr; }
};
constexpr int a = Number{};
static_assert(a == 1);
constexpr const int b = Number{};
static_assert(b == 1);
constexpr int via(const int& r) { return r; }
static_assert(via(Number{}) == 1);
constexpr long c = Number{};
static_assert(c == 0);
const int* p = Address{};
int* q = Address{};
