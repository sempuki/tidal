// tidal-rules: VOL-2
// tidal-expect: run
// tidal-note: The destructor of a volatile object runs with a this that is not volatile. So the destructor can call get, which is not a volatile member function, and its read of v is not an access through a glvalue that VOL-6 covers.
int seen = 0;
struct S {
  int v = 4;
  int get() { return v; }
  ~S() { seen = get(); }
};
int run() { { volatile S s; } return seen; }
int main() { return run() == 4 ? 0 : 1; }
