// tidal-rules: NAM-37 PFX-11
// tidal-expect: reject
// tidal-error-gcc: invalid use of
// tidal-error-clang: cannot refer to type member
// tidal-error-edg: type name is not allowed
// tidal-note: Member lookup finds the type alias Count, and a member access must not name a type.
struct Meter { using Count = int; };
int main() { Meter m; return m.Count; }
