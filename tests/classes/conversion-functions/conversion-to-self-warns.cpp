// tidal-rules: CVF-6
// tidal-expect: reject
// tidal-flags: -Wall -Werror
struct S { operator S() const; };
