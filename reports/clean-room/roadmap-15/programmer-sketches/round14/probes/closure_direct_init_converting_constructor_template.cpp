struct Ref {
  template <class F>
  Ref(F&) {}
};
int main() {
  auto f = [] {};
  Ref r(f);
  (void)r;
}
