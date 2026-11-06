template <class T> struct Maybe {
  T v;
  template <class Self> auto&& value(this Self&& self) { return static_cast<Self&&>(self).v; }
};
Maybe<int> make() { return {7}; }
int main() {
  auto&& r = make().value();  // binds to a member of a temporary that dies here
  return r;
}
