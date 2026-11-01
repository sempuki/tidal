// tidal-rules: DEP-14
// tidal-expect: run
struct Fixed { int m = 1; };
struct Extra { int m = 2; };
template <class T> struct Both : Fixed, T {
  int get() { return m; }
};
int main() { return Both<Extra>{}.get() == 1 ? 0 : 1; }
