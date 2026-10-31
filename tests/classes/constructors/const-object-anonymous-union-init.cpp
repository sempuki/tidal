// tidal-rules: CTOR-24
// tidal-expect: run
// tidal-expect-clang-18: reject
// tidal-error-clang-18: const type .* without a user-provided default constructor
// tidal-note: Q-0126. Clang 18 rejects a const object whose anonymous union member gets its value from a default member initializer. The other compilers accept it.
struct Cell {
  int tag = 1;
  union { int whole = 7; float part; };
};
const Cell cell;
int main() { return cell.tag == 1 && cell.whole == 7 ? 0 : 1; }
