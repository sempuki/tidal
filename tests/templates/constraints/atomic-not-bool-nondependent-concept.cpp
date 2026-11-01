// tidal-rules: CNS-39 CNS-9
// tidal-expect: reject
// tidal-expect-gcc: accept
// tidal-error-clang: atomic constraint must be of type 'bool'
// tidal-error-edg: atomic constraint must have type bool
// tidal-note: The concept is never used. Clang and EDG check the definition; GCC checks a concept only when satisfaction reaches the operand (Q-0263).
template <class T> concept Odd = (sizeof(T) > 100) && (sizeof(T));
int main() { return 0; }
