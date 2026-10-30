// tidal-rules: RFOR-2
// tidal-expect: reject
struct Incomplete;
extern Incomplete items[];
void f() {
  for (auto& v : items) (void)v;
}
