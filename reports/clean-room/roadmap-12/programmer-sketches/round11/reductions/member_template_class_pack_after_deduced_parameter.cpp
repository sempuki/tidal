// A member template whose parameter list expands the class's own pack after
// a deduced parameter. From signal.cpp.
template <class... Args> struct Sig {
  template <class T> T collect(T init, Args... args) { return init + (args + ...); }
};
int main() {
  Sig<int> s;
  return s.collect(1, 2) == 3 ? 0 : 1;
}
