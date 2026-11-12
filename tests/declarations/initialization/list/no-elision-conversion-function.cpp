// tidal-rules: LIST-4 CVF-1 DED-31
// tidal-expect: accept
// tidal-note: Front-end ledger row G183. Each value converts to the member's aggregate type, so the value initializes the whole member and the next value goes to the next member. That holds for a derived class, a class with a conversion function, a class that also converts to int, and a conversion function template.
struct Range { int low; int high; };
struct Span { Range range; int step; };
struct Wide : Range {};
struct ToRange { constexpr operator Range() const { return {7, 8}; } };
struct Both {
  constexpr operator Range() const { return {7, 8}; }
  constexpr operator int() const { return 1; }
};
struct Any {
  template <class T> constexpr operator T() const {
    if constexpr (sizeof(T) == sizeof(int)) return T(1);
    else return T{7, 8};
  }
};
constexpr Span derived{Wide{{7, 8}}, 5};
constexpr Span converted{ToRange{}, 5};
constexpr Span both{Both{}, 5};
constexpr Span any{Any{}, 5};
constexpr Span any_twice{Any{}, Any{}};
static_assert(derived.range.low == 7 && derived.range.high == 8 && derived.step == 5);
static_assert(converted.range.low == 7 && converted.range.high == 8 && converted.step == 5);
static_assert(both.range.low == 7 && both.range.high == 8 && both.step == 5);
static_assert(any.range.low == 7 && any.range.high == 8 && any.step == 5);
static_assert(any_twice.range.low == 7 && any_twice.step == 1);
