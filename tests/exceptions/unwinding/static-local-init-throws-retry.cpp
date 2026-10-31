// tidal-rules: UNW-4
// tidal-expect: run
int trace = 0;
void mark(int d) { trace = trace * 10 + d; }
struct T { int id; T(int i) : id(i) { mark(i); } ~T() { mark(id); } };

int attempts = 0;
struct Once { Once() { ++attempts; if (attempts == 1) throw 0; mark(5); } };
int get() { static Once once; return attempts; }
int main() {
  try { get(); } catch (int) { mark(1); }
  int second = get();
  int third = get();
  return second == 2 && third == 2 && trace == 15 ? 0 : 1;
}
