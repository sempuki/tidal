// tidal-rules: TPL-20 TPL-24
// tidal-expect: reject
// tidal-error-clang: too many template parameters in template redeclaration
// tidal-error-gcc: no declaration matches
// tidal-error-edg: declaration is incompatible with function template
// tidal-note: The definition gives one head where the member template of a class template needs two.
template <class T> struct Box {
  template <class U> U as() const;
};
template <class T, class U> U Box<T>::as() const { return U(); }
