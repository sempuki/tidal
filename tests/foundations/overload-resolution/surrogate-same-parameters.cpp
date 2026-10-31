// tidal-rules: OVL-36 OVL-6
// tidal-expect: reject
// tidal-expect-gcc: accept
// tidal-error-clang: call to object of type .Thing. is ambiguous
// tidal-error-edg: more than one
// tidal-note: Q-0139. Two surrogates with one parameter list differ only in the conversion function. The object's sequences use different conversion functions, so they tie (ICS-25). GCC chooses one.
using Fn = int (*)(int);
struct Thing {
  operator Fn();
  operator Fn() const;
};
Thing t;
int r = t(1);
