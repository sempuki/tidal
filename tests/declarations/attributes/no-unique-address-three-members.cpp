// tidal-rules: ATTR-16
// tidal-expect: run
// tidal-note: Three no_unique_address members of one empty type, between members of other types. Each pair has distinct addresses.
struct Tag {};
struct Three {
  int a;
  [[no_unique_address]] Tag x;
  [[no_unique_address]] Tag y;
  char b;
  [[no_unique_address]] Tag z;
};
int main() {
  Three t;
  const void* px = &t.x;
  const void* py = &t.y;
  const void* pz = &t.z;
  return px != py && py != pz && px != pz ? 0 : 1;
}
