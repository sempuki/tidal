// tidal-rules: ATTR-18
// tidal-expect: reject
// tidal-expect-clang: accept
// tidal-error-gcc: expected unqualified-id before .const.
// tidal-error-edg: expected an identifier
// tidal-note: Q-0123. Clang accepts an attribute before the cv-qualifiers of a pointer; GCC and EDG reject it.
int i = 0;
int * [[]] const p = &i;
