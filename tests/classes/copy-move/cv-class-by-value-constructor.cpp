// tidal-rules: CPY-15
// tidal-expect: reject
// tidal-error-clang: copy constructor must pass its first argument by reference
// tidal-error-gcc: invalid constructor
// tidal-error-edg: copy constructor for class
// tidal-note: Gap review RV6-A-18. A first parameter of type const T or volatile T, by value, is a first parameter of type T for CPY-15. Every compiler rejects both constructors.
struct Fixed { Fixed() {} Fixed(const Fixed); };
struct Shared { Shared() {} Shared(volatile Shared, int = 0); };
