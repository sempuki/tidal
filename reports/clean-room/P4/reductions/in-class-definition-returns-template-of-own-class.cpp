// tidal-fe: "rejected [CLS-1] at line 1: data member of incomplete class type".
// The full program stopped with [DCL-20] (array of incomplete class type) for the same shape.
// g++ and clang++ run it, exit 0. A member function defined in its class
// returns a class template specialization over that class (FN-12, CLS-2, INST-1).
template <class T> struct Box { T item; };
struct Money {
  int cents;
  Box<Money> boxed() const { return {*this}; }
};
int main() { return Money{3}.boxed().item.cents == 3 ? 0 : 1; }
