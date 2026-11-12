// tidal-rules: LIST-12 LIST-4
// tidal-expect: reject
// tidal-error-clang: excess elements in struct initializer
// tidal-error-gcc: too many initializers
// tidal-error-edg: too many initializer values
// tidal-note: Front-end ledger row G183. Each value converts to the member's aggregate type through a conversion function template, so the first value initializes the whole first member and the second value the second member. The third value has no member.
struct Range { int low; int high; };
struct Span { Range range; int step; };
struct Any {
  template <class T> constexpr operator T() const {
    if constexpr (sizeof(T) == sizeof(int)) return T(1);
    else return T{7, 8};
  }
};
constexpr Span span{Any{}, Any{}, Any{}};
