// tidal-rules: UNW-1
// tidal-expect: run
int trace = 0;
void mark(int d) { trace = trace * 10 + d; }
struct T { int id; T(int i) : id(i) { mark(i); } ~T() { mark(id); } };

int main() {
  T outer(1);
  try { throw 0; }
  catch (int) { T h(2); mark(3); }
  mark(4);
  return trace == 12324 ? 0 : 1;
}
