// tidal-rules: OVL-20 OVL-18
// tidal-expect: reject
// tidal-error-clang: address of overloaded function .pick. does not match required type
// tidal-error-gcc: no matches converting function .pick. to type
// tidal-error-edg: no instance of overloaded function .pick. matches the required type
// tidal-note: A function without noexcept does not match a noexcept target.
void pick(int);
void pick(long);
void (*p)(int) noexcept = pick;
