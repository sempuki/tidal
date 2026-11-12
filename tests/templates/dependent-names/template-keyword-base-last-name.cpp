// tidal-rules: DEP-25
// tidal-expect: run
// tidal-note: RV10-1. The keyword template comes before the last template-id of a qualified base name, and of the same name in a member initializer list, in a class template and outside one. Every compiler accepts both forms.
struct Parts {
  template <class U> struct Part {
    U value = 1;
    Part() = default;
    Part(U v) : value(v) {}
  };
};
template <class T> struct Derived : T::template Part<int> {
  Derived() : T::template Part<int>(2) {}
};
struct Plain : Parts::template Part<int> {
  Plain() : Parts::template Part<int>{3} {}
};
int main() { return Derived<Parts>().value + Plain().value == 5 ? 0 : 1; }
