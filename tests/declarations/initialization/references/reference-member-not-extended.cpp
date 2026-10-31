// tidal-rules: REF-4
// tidal-expect: run
int ended = 0;
int target = 5;
struct Holder { const int& r; ~Holder() { ++ended; } };
int main() {
  const int& x = Holder{target}.r;
  int seen = ended;          // 1 if the Holder ended at the semicolon
  return seen == 1 && x == 5 ? 0 : 1;
}
