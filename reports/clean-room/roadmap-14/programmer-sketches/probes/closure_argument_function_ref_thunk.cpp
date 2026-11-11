struct Ref {
  void* obj;
  int (*call)(void*, int);
  template <class F>
  Ref(F& f) : obj(&f), call([](void* o, int x) { return (*static_cast<F*>(o))(x); }) {}
  int operator()(int x) const { return call(obj, x); }
};
int use(Ref r) { return r(3); }
int main() {
  auto upper = [](int x) { return 2 * x; };
  return use(upper) - 6;
}
