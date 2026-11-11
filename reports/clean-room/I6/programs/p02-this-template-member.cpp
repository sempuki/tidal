// DEP-24, DEP-11: a member template of a dependent base through this->template.
template <class T> struct Base {
  template <int N> int get() const { return N; }
};
template <class T> struct Derived : Base<T> {
  int run() const { return this->template get<3>() + Derived::template get<4>(); }
};
int main() { return Derived<int>{}.run() == 7 ? 0 : 1; }
