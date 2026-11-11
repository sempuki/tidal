// A member template called through a parameter whose type is the current
// instantiation. GCC, Clang, and EDG need no `template` keyword; the front end
// rejects with DEP-24.
template <class T> struct Box {
  T value{};
  template <class U> U as() const { return static_cast<U>(value); }
  static int read(const Box& b) { return b.as<int>(); }
};
int main() {
  Box<double> b{2.5};
  return Box<double>::read(b) == 2 ? 0 : 1;
}
