// tidal-rules: TPL-25
// tidal-expect: reject
// tidal-error-clang: .virtual. cannot be specified on member function templates
// tidal-error-gcc: templates may not be .virtual.
// tidal-error-edg: .virtual. is not allowed in a function template declaration
struct Shape {
  template <class T> virtual T area() const;
};
