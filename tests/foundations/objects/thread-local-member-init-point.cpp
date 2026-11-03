// tidal-rules: OBJ-11
// tidal-expect: run
// tidal-note: A thread_local static data member with a constructor. Construction before main and at first use are both allowed.
int trace = 0;
struct T { int id; T(int i) : id(i) { trace = trace * 10 + i; } };
struct Holder { static thread_local T value; };
thread_local T Holder::value(6);
int main() {
  int before = trace;
  int id = Holder::value.id;
  return (before == 0 || before == 6) && id == 6 && trace == 6 ? 0 : 1;
}
