// tidal-rules: REF-6
// tidal-expect: accept
// tidal-expect-gcc: reject
// tidal-error-gcc: invalid conversion from .<brace-enclosed initializer list>. to .int \[1\].
// tidal-note: Q-0164. A non-const reference to int[] binds directly to an int[3] lvalue in braces. GCC makes a temporary int[1] from the list instead and rejects it.
int store[3];
int (&whole)[]{store};
int (&same)[] = store;
