// tidal-rules: ICS-37
// tidal-expect: accept
// tidal-note: Each class has two conversion functions whose results both convert to the target by conversion rank. Only the nearest source decides.
template <class T, class U> constexpr bool same = false;
template <class T> constexpr bool same<T, T> = true;
struct One {};
struct Two {};
struct Base { int x; };
struct Middle : Base {};
struct Derived : Middle {};
Derived dd;
Middle mm;
struct Ptr { constexpr operator Derived*() const { return nullptr; } constexpr operator Middle*() const { return &mm; } };
static_assert(static_cast<Base*>(Ptr{}) == &mm);
struct Ref { constexpr operator Derived&() const { return dd; } constexpr operator Middle&() const { return mm; } };
constexpr Base& r = Ref{};
static_assert(&r == &mm);
struct Mem { constexpr operator int Base::*() const { return nullptr; } constexpr operator int Middle::*() const { return &Middle::x; } };
constexpr int Derived::* m = Mem{};
static_assert(m != nullptr);
