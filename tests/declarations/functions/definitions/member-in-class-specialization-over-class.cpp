// tidal-rules: FN-12 CLS-2 INST-1 INST-30
// tidal-expect: run
// tidal-note: P4-1. Coin defines wrap and unwrap in its class. Their return and parameter type Holder<Coin> has a member of type Coin, so Holder<Coin> can be complete only after Coin is. For a member function defined in its class, FN-12 checks the types at the end of the class, where Coin is complete, and the point of instantiation of Holder<Coin> is right after Coin (INST-30). GCC, Clang, and EDG accept.
template <class T> struct Holder { T value; };
struct Coin {
  int cents;
  Holder<Coin> wrap() const { return Holder<Coin>{*this}; }
  int unwrap(Holder<Coin> h) const { return h.value.cents; }
};
int main() {
  Coin c{5};
  return c.unwrap(c.wrap()) - 5;
}
