// tidal-rules: DEP-11
// tidal-expect: run
int count = 5;
template <class T> struct Base { int count = 1; };
template <class T> struct Derived : Base<T> {
  int get() { return this->count + Derived::count + Base<T>::count; }
};
int main() { return Derived<int>{}.get() == 3 ? 0 : 1; }
