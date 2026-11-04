// tidal-rules: CTOR-12
// tidal-expect: reject
// tidal-note: The mem_init of Base uses this as the operand of a dynamic_cast before Base is initialized, so the behavior is undefined and the constant evaluation fails.
struct Base { const void* where; constexpr Base(const void* p) : where(p) {} };
struct Derived : Base {
  constexpr Derived() : Base(dynamic_cast<const void*>(this)) {}
  constexpr virtual ~Derived() {}
};
constexpr bool build() { Derived d; return d.where != nullptr; }
static_assert(build());
