// tidal-rules: TSP-5
// tidal-expect: reject
// tidal-error-clang: templates can only be declared in namespace or class scope
// tidal-error-clang-18: expected expression
// tidal-error-gcc: a template declaration cannot appear at block scope
// tidal-error-edg: a template declaration is not allowed here
template <class T> struct Box {};
void use() {
  template <> struct Box<int> {};
}
