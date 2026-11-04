// tidal-rules: NAM-27
// tidal-expect: accept
// tidal-note: The argument takes the address of an overload set. It adds what the set adds, so the parameter type m::B adds m.
namespace m { struct B {}; int take(void (*)(B)); }
void h(m::B);
int r = take(&h);
