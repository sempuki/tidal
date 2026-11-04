// tidal-rules: ADD-14 ADD-10
// tidal-expect: accept
// tidal-interpreter: run-fail
// tidal-note: Q-0049. A B* into an array of D counts its B as an array of one, so the move by two positions is undefined (ADD-14, a case of ADD-10). No sanitizer in the matrix stops the run, so the matrix compilers only build the program. The interpreter must stop with ADD-14 or ADD-10 (decisions 0052, 0053).
struct B { int b; };
struct D : B { int d; };
int main() {
  D arr[3]{};
  B* p = arr;
  B* q = p + 2;   // undefined: beyond the B and the position one past it
  return q != p ? 0 : 1;
}
