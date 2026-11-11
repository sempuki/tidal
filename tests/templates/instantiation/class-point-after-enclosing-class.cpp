// tidal-rules: INST-30 CLS-2
// tidal-expect: run
// tidal-note: RV9-2. Holder<Coin> is first needed in a member function body of Coin, in a default argument, and in a member function body of a nested class. Its point is right after the definition of Coin, so Coin is complete there, with the member after the bodies. GCC, Clang, and EDG accept.
template <class T> struct Holder {
  T value;
  static constexpr int size = sizeof(T);
};
struct Coin {
  int cents;
  int held() const { return Holder<Coin>::size; }
  int given(int n = sizeof(Holder<Coin>)) const { return n; }
  struct Inner {
    int outer() const { return Holder<Coin>::size; }
  };
  int extra;
};
int main() {
  Coin c{};
  return c.held() == sizeof(Coin) && c.given() == sizeof(Coin) &&
                 Coin::Inner{}.outer() == sizeof(Coin) && sizeof(Coin) == 2 * sizeof(int)
             ? 0
             : 1;
}
