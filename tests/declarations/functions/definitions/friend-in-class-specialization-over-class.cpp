// tidal-rules: FN-12 CLS-2 INST-1
// tidal-expect: run
// tidal-note: RV9-1. Coin defines the friend function wrap in its class. Its return type Holder<Coin> has a member of type Coin, so Holder<Coin> can be complete only after Coin is. FN-12 checks the types of a friend function defined in a class where the class ends. GCC, Clang, and EDG accept.
template <class T> struct Holder { T value; };
struct Coin {
  int cents;
  friend Holder<Coin> wrap(Coin c) { return Holder<Coin>{c}; }
};
int main() {
  return wrap(Coin{5}).value.cents - 5;
}
