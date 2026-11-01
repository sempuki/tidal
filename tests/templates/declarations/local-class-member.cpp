// tidal-rules: TPL-2
// tidal-expect: reject
// tidal-error-clang: templates cannot be declared inside of a local class
// tidal-error-gcc: invalid declaration of member template in local class
// tidal-error-edg: a template declaration is not allowed here
// tidal-note: A member function template in a class that a function defines.
int use() {
  struct Local {
    template <class T> T get(T v) { return v; }
  };
  return Local().get(1);
}
