// tidal-rules: FN-4
// tidal-expect: reject
// tidal-error-clang: main cannot be declared as (a variable in the )?global( scope| variable)
// tidal-error-gcc: cannot declare .::main. to be a global variable
// tidal-error-edg: main. cannot be used as a global variable name
int main = 0;
