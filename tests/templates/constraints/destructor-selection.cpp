// tidal-rules: CNS-19
// tidal-expect: run
// tidal-note: Both destructors are satisfied for int, and the second is more constrained.
template <class T> concept Big = sizeof(T) > 1;
template <class T> concept Bigger = Big<T> && sizeof(T) > 2;
int calls = 0;
template <class T> struct Box {
  ~Box() requires Big<T> { calls = 1; }
  ~Box() requires Bigger<T> { calls = 2; }
};
int main() {
  { Box<int> box; }
  return calls == 2 ? 0 : 1;
}
