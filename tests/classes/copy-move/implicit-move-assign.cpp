// tidal-rules: CPY-5
// tidal-expect: run
int copy_assigns = 0;
int move_assigns = 0;
struct M {
  M& operator=(const M&) { ++copy_assigns; return *this; }
  M& operator=(M&&) { ++move_assigns; return *this; }
};
struct S { M m; };
int main() { S a, b; a = static_cast<S&&>(b); return copy_assigns == 0 && move_assigns == 1 ? 0 : 1; }
