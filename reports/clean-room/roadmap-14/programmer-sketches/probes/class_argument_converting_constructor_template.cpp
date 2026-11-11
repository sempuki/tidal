struct C {};
struct Ref {
  template <class F>
  Ref(F) {}
};
int use(Ref) { return 0; }
int main() {
  return use(C{});
}
