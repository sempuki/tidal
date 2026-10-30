// tidal-rules: RFOR-4
// tidal-expect: run
// tidal-expect-gcc-14: run-fail
// tidal-expect-clang-18: run-fail
// tidal-note: Q-0020. GCC 14 and Clang 18 predate the C++23 rule that keeps temporaries in the range expression alive for the whole loop.
int destroyed = 0;
struct Owner {
  int data[3] = {1, 2, 3};
  ~Owner() { ++destroyed; }
  int (&items())[3] { return data; }
};
Owner make() { return {}; }
int main() {
  int seen_destroyed = -1;
  for (int v : make().items()) { seen_destroyed = destroyed; (void)v; }
  return seen_destroyed == 0 ? 0 : 1;
}
