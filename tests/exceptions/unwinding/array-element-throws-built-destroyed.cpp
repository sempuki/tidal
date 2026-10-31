// tidal-rules: UNW-2
// tidal-expect: run
int trace = 0;
void mark(int d) { trace = trace * 10 + d; }
struct T { int id; T(int i) : id(i) { mark(i); } ~T() { mark(id); } };

int fail_at = 3;
struct E { int id; E(int i) : id(i) { if (i == fail_at) throw 0; mark(i); } ~E() { mark(id); } };
int main() {
  try { E arr[4] = {1, 2, 3, 4}; }
  catch (int) { mark(9); }
  return trace == 12219 ? 0 : 1;
}
