// Migration: each affected test's comma, with the left operand cast to void.
int destroyed = 0;
struct Note { int v; ~Note() { ++destroyed; } };
Note make(int v) { return Note{v}; }
int trace[8]; int n = 0;
struct Probe { int id; Probe(int i) : id(i) { trace[n++] = id; } ~Probe() { trace[n++] = -id; } };
template <int N> constexpr int value = N;
static_assert(((void)1, 2) == 2);                         // COMMA-3
static_assert(value<((void)1, 2)> == 2);                  // COMMA-3, template argument
int a[((void)1, 2)];                                      // COMMA-8
static_assert(sizeof(a) == 2 * sizeof(int));
constexpr const int& last = ((void)0, 7);                 // CE-14
enum E { e1 = ((void)1, 5), e2 };                         // COMMA-4
int x = 5;
int main() {
  {
    const Note& c = ((void)0, make(4));                   // REF-4
    if (destroyed != 0) return 1;
    if (c.v != 4) return 2;
  }
  if (destroyed != 1) return 3;
  int r = ((void)Probe(1), (void)Probe(2), 5);            // COMMA-1
  trace[n++] = 9;
  if (!(r == 5 && n == 5 && trace[0] == 1 && trace[1] == 2 && trace[2] == -2 &&
        trace[3] == -1 && trace[4] == 9)) return 4;
  int* p = &((void)1, x);                                 // COMMA-2
  int& rr = ((void)0.5, x);
  rr = 6;
  if (!(p == &x && x == 6)) return 5;
  int arr[3] = {5, 6, 7};
  if (arr[((void)0, 2)] != 7) return 6;                   // COMMA-6
  int y = 2;
  ((void)x, y) = 9;                                       // COMMA-2
  if (y != 9) return 7;
  return e2 == 6 && last == 7 ? 0 : 8;
}
