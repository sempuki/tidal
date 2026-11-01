// tidal-rules: CNS-39 CNS-9
// tidal-expect: reject
// tidal-error-clang: atomic constraint must be of type 'bool'
// tidal-error-gcc: does not have type .bool.
// tidal-error-edg: atomic constraint must have type bool
// tidal-note: The template is never used. The type of (sizeof(T)) does not depend on T, so the definition is checked.
template <class T>
  requires (sizeof(T) > 100) && (sizeof(T))
void wide(T) {}
int main() { return 0; }
