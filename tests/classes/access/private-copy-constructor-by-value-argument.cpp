// tidal-rules: ACC-3 CPY-9
// tidal-expect: reject
// tidal-error-clang: calling a private constructor
// tidal-error-gcc: is private within this context
// tidal-error-edg: is inaccessible
// tidal-note: G154 (front-end report 23). The parameter of take is a Ticket, and the argument is a Ticket lvalue, so the parameter's initialization selects the copy constructor (CPY-9). The copy constructor is private, so the call is not valid (ACC-3).
struct Ticket {
  Ticket() = default;
private:
  Ticket(const Ticket&) = default;
};
void take(Ticket) {}
int main() {
  Ticket a;
  take(a);
  return 0;
}
