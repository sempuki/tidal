// tidal-rules: ICS-8 LIST-2
// tidal-expect: reject
// tidal-error-clang: cannot be narrowed
// tidal-error-gcc: narrowing conversion
// tidal-error-edg: narrowing conversion
// tidal-note: Narrowing does not stop the standard sequence to int, which beats the user-defined one. The narrowing then fails (LIST-2).
template <class T, class U> constexpr bool same = false;
template <class T> constexpr bool same<T, T> = true;
struct One {};
struct Two {};
struct Real { Real(double); };
double d = 1.5;
One take(int);
Two take(Real);
auto r = take({d});
