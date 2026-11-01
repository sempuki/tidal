// tidal-rules: CE-14
// tidal-expect: run
int read() {
  static constexpr const int& limit = 4;
  static constexpr const int* where = &limit;
  return *where;
}
int main() { return read() == 4 ? 0 : 1; }
