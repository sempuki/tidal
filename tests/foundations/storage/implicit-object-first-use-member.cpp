// tidal-rules: STG-8
// tidal-expect: run
// tidal-flags: -O0 -fsanitize=address,undefined -fno-sanitize-recover=all
// tidal-note: The first use names int, and a later use names Pair. Only a set that holds a Pair gives defined behavior, so an array of int is not the set that STG-8 creates. The pointer from the first use points to the member first of that Pair, also after the later use.
struct Pair { int first; int second; };
int main() {
  void* raw = ::operator new(sizeof(Pair));
  int* head = static_cast<int*>(raw);   // the member first of the Pair
  *head = 1;
  Pair* pair = static_cast<Pair*>(raw);
  pair->second = 2;
  *head += 10;                          // still the member first
  int result = pair->first + pair->second == 13 && head == &pair->first ? 0 : 1;
  ::operator delete(raw);
  return result;
}
