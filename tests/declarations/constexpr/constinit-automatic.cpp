// tidal-rules: CINIT-2
// tidal-expect: reject
// tidal-error-clang: local variable cannot be declared .constinit
// tidal-error-gcc: constinit. can only be applied to a variable with static or thread storage duration
// tidal-error-edg: constinit. is only valid for declarations of variables with static or thread storage duration
void f() {
  constinit int x = 1;
}
