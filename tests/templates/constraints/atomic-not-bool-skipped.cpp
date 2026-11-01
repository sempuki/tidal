// tidal-rules: CNS-39 CNS-9
// tidal-expect: reject
// tidal-expect-gcc: accept
// tidal-error-clang: atomic constraint must be of type 'bool'
// tidal-error-gcc: not .bool.|does not have type .bool.
// tidal-error-edg: atomic constraint must have type bool
// tidal-note: The type of (sizeof(T)) is the same for every T and is not bool. Clang and EDG reject the concept, GCC skips the operand and accepts (Q-0263).
template <class T> concept Odd = (sizeof(T) > 100) && (sizeof(T));
static_assert(!Odd<int>);
