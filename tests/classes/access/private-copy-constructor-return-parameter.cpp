// tidal-rules: ACC-3 CPY-9
// tidal-expect: reject
// tidal-error-clang: calling a private constructor
// tidal-error-gcc: is private within this context
// tidal-error-edg: is inaccessible
// tidal-note: G154 (front-end report 23). The return statement initializes the Ticket result from a const Ticket lvalue, which selects the copy constructor (CPY-9). The copy constructor is private, so the return is not valid (ACC-3).
struct Ticket {
  Ticket() = default;
private:
  Ticket(const Ticket&) = default;
};
Ticket again(const Ticket& t) { return t; }
int main() {
  Ticket a;
  (void)again(a);
  return 0;
}
