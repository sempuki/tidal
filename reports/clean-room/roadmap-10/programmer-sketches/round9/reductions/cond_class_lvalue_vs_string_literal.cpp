// The conditional operator: a class lvalue against a string literal that
// converts to the class through a converting constructor (COND-16).
struct Str {
  const char* p;
  Str(const char* s) : p(s) {}
};
int main(int argc, char**) {
  Str a = "loud";
  Str r = argc > 5 ? a : "quiet";
  return r.p[0] == 'q' ? 0 : 1;
}
