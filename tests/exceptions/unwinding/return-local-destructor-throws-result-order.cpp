// tidal-rules: UNW-1
// tidal-expect: run
// tidal-expect-clang: run-fail
// tidal-note: Q-0365. The return statement builds the result after first and second, and then the destructor of second throws. UNW-1 ends the result after the objects of the block, so first ends before the result. GCC follows this order. Clang never destroys the result, and the program returns 1 (documentation/decisions/pending.md, row 4).
int trace = 0;
void mark(int d) { trace = trace * 10 + d; }
struct Result { Result() {} Result(const Result&) {} ~Result() { mark(9); } };
struct Local {
  int id;
  ~Local() noexcept(false) { mark(id); if (id == 2) throw 0; }
};
Result make() { Local first{1}; Local second{2}; return Result(); }
int main() {
  try { Result r = make(); } catch (int) {}
  if (trace == 21) return 1;
  return trace == 219 ? 0 : 2;
}
