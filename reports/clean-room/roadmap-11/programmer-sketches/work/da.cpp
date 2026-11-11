template <class... Args> struct Sig {
  template <class T> T collect(T init, Args... args) { return init + (args + ...); }
};
int main() {
  Sig<int> s;
  return s.collect(1, 2) == 3 ? 0 : 1;
}
