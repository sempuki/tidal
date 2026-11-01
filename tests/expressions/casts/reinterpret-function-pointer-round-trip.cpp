// tidal-rules: CAST-6
// tidal-expect: run
int seen = 0;
void store(long v) { seen = static_cast<int>(v); }
using IntTaker = void (*)(int);
using LongTaker = void (*)(long);
int main() {
  IntTaker p = reinterpret_cast<IntTaker>(&store);
  LongTaker back = reinterpret_cast<LongTaker>(p);
  back(5);
  return back == &store && seen == 5 ? 0 : 1;
}
