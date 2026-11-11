// tidal-rules: INST-30 INST-33
// tidal-expect: reject
// tidal-error-clang: undeclared identifier 'extra'
// tidal-error-gcc: .extra. was not declared in this scope
// tidal-error-edg: identifier "extra" is undefined
// tidal-note: RV9-2. The body of wrap needs Holder<Coin>, whose point of instantiation is right after the definition of Coin. The member type tag is made with the class and needs extra, which is declared only after that point. So the class is not valid. GCC, Clang, and EDG reject.
template <class T> struct Holder {
  T value;
  using tag = decltype(extra(value));
};
struct Coin {
  int cents;
  Holder<Coin> wrap() const { return Holder<Coin>{*this}; }
};
int extra(Coin);
