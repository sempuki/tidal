template <class A> struct Sig {
  template <class T> T collect(T init, A a) { return init + a; }
};
int main() {
  Sig<int> s;
  return s.collect(1, 2) == 3 ? 0 : 1;
}
