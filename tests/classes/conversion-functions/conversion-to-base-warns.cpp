// tidal-rules: CVF-6
// tidal-expect: reject
// tidal-flags: -Wall -Werror
struct B {};
struct D : B { operator B() const; };
