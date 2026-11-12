// tidal-rules: ENUM-20
// tidal-expect: reject
// tidal-error-clang: using-enum cannot name a dependent type
// tidal-error-gcc: using enum. of dependent type
// tidal-error-edg: is not an enum type
// tidal-note: RV10-3. A using enum declaration in a function template names a dependent type. Every compiler rejects it.
struct Palette { enum class Color { red, green }; };
template <class T> int count() { using enum T::Color; return 0; }
int total = count<Palette>();
