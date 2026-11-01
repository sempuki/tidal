// tidal-rules: OVL-9 OVL-36
// tidal-expect: reject
// tidal-error-clang: use of overloaded operator .!. is ambiguous
// tidal-error-gcc: ambiguous overload for .operator!.
// tidal-error-edg: more than one operator .!. matches
// tidal-note: operator!(Gate) needs the converting constructor of Gate. The built-in candidate (bool) needs the explicit conversion function to bool. Both sequences are user-defined, so neither candidate is better.
struct Flag { explicit operator bool() const; };
struct Gate { Gate(Flag); };
int operator!(Gate);
void use(Flag f) { !f; }
