// tidal-rules: TPL-2
// tidal-expect: reject
// tidal-error-clang: templates cannot be declared inside of a local class
// tidal-error-gcc: invalid declaration of member template in local class
// tidal-error-edg: a template friend declaration cannot be declared in a local class
template <class T> struct Box {};
void use() {
  struct Local { template <class T> friend struct Box; };
}
