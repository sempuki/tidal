// tidal-rules: ACC-3 CPY-9
// tidal-expect: reject
// tidal-error-clang: calling a private constructor
// tidal-error-gcc: is private within this context
// tidal-error-edg: is inaccessible
// tidal-note: G154 (front-end report 23). The copy-initialization from a Ticket lvalue selects the copy constructor (CPY-9). The copy constructor is private, and main is not a member or friend of Ticket, so the initialization is not valid (ACC-3).
struct Ticket {
  Ticket() = default;
private:
  Ticket(const Ticket&) = default;
};
int main() {
  Ticket a;
  Ticket b = a;
  (void)b;
  return 0;
}
