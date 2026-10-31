// tidal-rules: ICS-8 REF-8
// tidal-expect: reject
// tidal-error-clang: reference cannot bind to bit-field
// tidal-error-gcc: cannot bind bit-field
// tidal-error-edg: address of a bit field
// tidal-note: The bit-field forms an exact-match sequence to int&, which beats the conversion to long. The binding then fails (REF-8).
template <class T, class U> constexpr bool same = false;
template <class T> constexpr bool same<T, T> = true;
struct One {};
struct Two {};
struct Flags { int mode : 3; };
Flags flags{};
One take(int&);
Two take(long);
auto r = take(flags.mode);
