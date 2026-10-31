// tidal-rules: DER-12
// tidal-expect: reject
// tidal-expect-edg: accept
// tidal-error-clang: not allowed in struct with virtual base class
// tidal-error-gcc: has virtual base classes
// tidal-note: Q-0085. EDG accepts a constexpr destructor in a class with a virtual base. Clang and GCC reject it.
struct A {};
struct D : virtual A { constexpr ~D() {} };
