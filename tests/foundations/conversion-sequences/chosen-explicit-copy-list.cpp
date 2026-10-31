// tidal-rules: ICS-8 ILC-8
// tidal-expect: reject
// tidal-expect-clang: accept
// tidal-expect-edg: accept
// tidal-error-gcc: would use explicit constructor
// tidal-note: Q-0132. GCC keeps the explicit constructor, so the sequence to Strict exists and take(Strict, int) is chosen. Clang and EDG have no sequence to Strict and choose take(Loose, long).
template <class T, class U> constexpr bool same = false;
template <class T> constexpr bool same<T, T> = true;
struct One {};
struct Two {};
struct Strict { explicit Strict(int, int); };
struct Loose { Loose(long, long); };
One take(Strict, int);
Two take(Loose, long);
auto r = take({1, 2}, 0);
