// tidal-rules: UNEV-3
// tidal-expect: accept
// tidal-flags: -Werror -Wunused-value
// tidal-note: X26-3. Each discarded expression calls a function, a constructor, or a literal operator, writes an object, reads a volatile object, or allocates. Clang and GCC give no warning in an expression statement, as a left operand of a comma, or as an iteration expression.
struct Token { int id; Token(int i) : id(i) {} ~Token() {} };
int next();
void reset();
int (*source)() = next;
int operator""_id(unsigned long long v) { return int(v); }
volatile int port = 0;
int run(int a, int b, bool c) {
  port;
  Token(1);
  Token{2};
  new int;
  4_id;
  ++a;
  a++;
  a = 1;
  a += 2;
  next();
  source();
  reset();
  [] { return 1; }();
  (void)b;
  c ? next() : next();
  int r = (port, a = 3, next(), b);
  for (int k = 0; k < 2; ++k, Token(3)) r += k;
  return r;
}
