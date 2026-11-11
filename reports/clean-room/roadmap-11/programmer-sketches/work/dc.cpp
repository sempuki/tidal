template <class... Args> struct Sig {
  template <class T> T collect(T init, Args... args) { return init; }
};
int main() {
  Sig<int> s;
  return s.collect(1, 2) == 1 ? 0 : 1;
}
