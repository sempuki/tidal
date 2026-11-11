// DEP-11, DEP-23: a using-declaration brings a value member of a dependent base.
template <class T> struct Base { int value = 5; int twice() const { return 2 * value; } };
template <class T> struct Derived : Base<T> {
  using Base<T>::value;
  using Base<T>::twice;
  int get() const { return value + twice(); }
};
int main() { return Derived<int>{}.get() == 15 ? 0 : 1; }
