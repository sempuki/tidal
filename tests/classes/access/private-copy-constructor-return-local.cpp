// tidal-rules: ACC-3 CPY-11
// tidal-expect: reject
// tidal-error-clang: calling a private constructor
// tidal-error-gcc: is private within this context
// tidal-error-edg: is inaccessible
// tidal-note: G154 (front-end report 23). The name of the local t in the return statement is an xvalue (CPY-11). Ticket has no move constructor, because it declares a copy constructor, so the private copy constructor serves the return, and the return is not valid (ACC-3).
struct Ticket {
  Ticket() = default;
private:
  Ticket(const Ticket&) = default;
};
Ticket fresh() {
  Ticket t;
  return t;
}
int main() {
  (void)fresh();
  return 0;
}
