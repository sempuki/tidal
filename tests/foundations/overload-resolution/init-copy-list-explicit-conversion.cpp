// tidal-rules: OVL-14 OVL-35
// tidal-expect: reject
// tidal-expect-clang: accept
// tidal-error-gcc: could not convert
// tidal-error-edg: no instance of constructor .Target::Target. matches the argument list
// tidal-note: Q-0145. In copy-list-initialization the copy constructor's parameter would need the explicit conversion function, which copy-initialization does not take. Clang takes it.
struct Target { Target(); };
struct Source { explicit operator Target() const; };
Source s;
Target t = {s};
