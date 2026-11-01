// tidal-rules: DEP-19
// tidal-expect: run
struct Has { static constexpr int count = 2; };
int scale = 3;
template <class T> int read() {
  int n(T::count);
  return n * T::count * scale;
}
int main() { return read<Has>() == 12 ? 0 : 1; }
