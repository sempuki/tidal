// tidal-rules: CAST-1 STG-16
// tidal-expect: run
// tidal-note: W20-5. A pointer to a class object converts to const void* and back to its own type with static_cast. The result points to the original object, so it equals the original pointer and reads the member.
struct Account { int balance; };
const void* erase(const Account* a) { return a; }
int main() {
  Account a{40};
  const void* hidden = erase(&a);
  const Account* back = static_cast<const Account*>(hidden);
  return back == &a && back->balance == 40 ? 0 : 1;
}
