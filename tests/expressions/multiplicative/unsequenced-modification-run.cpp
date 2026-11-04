// tidal-rules: MUL-14 SEQ-4
// tidal-expect: accept
// tidal-interpreter: run-fail
// tidal-note: The left operand writes i and the right operand reads it, with no sequencing between them (MUL-14, the case of SEQ-4 for the operands of a multiplicative operator). No sanitizer in the matrix stops the run, so the matrix compilers only build the program. The interpreter must stop with MUL-14 or SEQ-4 (decisions 0052, 0053).
int main() {
  int i = 2;
  int product = i++ * i;   // undefined: a write and a read of i, unsequenced
  return product == 6 ? 0 : 1;
}
