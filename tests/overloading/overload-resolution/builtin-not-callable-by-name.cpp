// tidal-rules: OVL-9 OVL-35
// tidal-expect: reject
// tidal-error-clang: no matching function for call to .operator\+.
// tidal-error-gcc: could not convert .1. from .int. to .Meter.
// tidal-error-edg: no suitable constructor exists to convert from "int" to "Meter"
// tidal-note: The call by name finds only operator+(Meter, int). The built-in candidate for int + int exists only inside an expression with +.
struct Meter {};
int operator+(Meter, int);
int sum = operator+(1, 2);
