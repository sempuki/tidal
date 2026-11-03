// tidal-rules: DTY-5
// tidal-expect: reject
// tidal-expect-gcc: accept
// tidal-error-clang: decltype.*(name a declaration|declarative nested name specifier)
// tidal-error-edg: decltype-qualified name is nonstandard in this declaration context
// tidal-note: Q-0065. The decltype is in the qualified name of a static data member definition. GCC accepts it. Clang and EDG reject it.
struct S { static int count; };
S s;
int decltype(s)::count = 0;
