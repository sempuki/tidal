// tidal-rules: OVL-15 OVL-35
// tidal-expect: reject
// tidal-error-clang: no viable conversion from .Source. to .const int.
// tidal-error-gcc: invalid initialization of reference of type .const int&. from expression of type .Source.
// tidal-error-edg: no suitable conversion function from "Source" to "const int" exists
// tidal-note: The same classes as reference-explicit-direct.cpp. Copy initialization does not take the explicit conversion function.
int value = 5;
struct Source { explicit operator int&() const { return value; } };
Source s;
const int& r = s;
