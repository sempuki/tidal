// DEP-15, DEP-16: a member of a nested class defined outside, names of the current instantiation.
template <class T> struct Outer {
  struct Inner {
    using type = int;
    type get() const;
  };
};
template <class T> typename Outer<T>::Inner::type Outer<T>::Inner::get() const {
  Inner::type a = 1;
  Outer<T>::Inner::type b = 2;
  return a + b;
}
int main() { return Outer<long>::Inner{}.get() == 3 ? 0 : 1; }
