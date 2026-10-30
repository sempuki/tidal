// tidal-rules: LNK-4
// tidal-expect: run
// tidal-sources: inline-one-entity.tu.cpp
inline int shared_inline = 5;
inline int twice(int v) { return v * 2; }
const int* other_address();
int (*other_function())(int);
int main() {
  return &shared_inline == other_address() && &twice == other_function() ? 0 : 1;
}
