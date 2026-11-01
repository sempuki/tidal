// tidal-rules: ICS-38
// tidal-expect: accept
// tidal-note: The braced literal has the referenced type, so each reference binds directly to the literal, which is an lvalue. The rvalue reference cannot bind to it, and the lvalue reference wins. The same holds for an array of unknown bound.
template <class T, class U> constexpr bool same = false;
template <class T> constexpr bool same<T, T> = true;
struct One {};
struct Two {};
One known(const char (&)[3]);
Two known(const char (&&)[3]);
static_assert(same<decltype(known({"ab"})), One>);
One unknown(const char (&)[]);
Two unknown(const char (&&)[]);
static_assert(same<decltype(unknown({"ab"})), One>);
