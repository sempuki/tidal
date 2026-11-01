// tidal-rules: DED-8 DED-35
// tidal-expect: reject
// tidal-error-clang: could not match
// tidal-error-gcc: no matching function for call to
// tidal-error-gcc-14: invalid conversion from
// tidal-error-edg: no instance of function template
// tidal-note: T&& is a parameter of a function pointer type, so it is no forwarding reference. T&& does not match int&, and the call has no viable function. GCC 14 reports a conversion error for the same call.
template <class T> void take(void (*)(T&&));
void sink(int&);
void use() { take(sink); }
