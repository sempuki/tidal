template <class T, class... Args> T collect(T init, Args... args) { return init + (args + ...); }
int main() {
  return collect<int, int>(1, 2) == 3 ? 0 : 1;
}
