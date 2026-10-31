// tidal-rules: NAM-21
// tidal-expect: accept
struct C { using type = int; void set(type); type held = 0; };
void C::set(type v) { held = v; }
