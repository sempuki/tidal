// tidal-rules: OVL-20
// tidal-expect: reject
// tidal-error-clang: incompatible initializer of type .<overloaded function type>.
// tidal-error-gcc: unable to deduce .auto. from
// tidal-error-edg: cannot deduce .auto. type
// tidal-note: auto gives no target, and the set has two functions.
void pick(int);
void pick(long);
auto p = &pick;
