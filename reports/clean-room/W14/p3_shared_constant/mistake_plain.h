// Deliberate mistake: a variable definition that is neither inline nor
// const, in a header that two units include. ODR-8 / LNK-6: one
// definition in the program. The linker reports the second one.
#ifndef W14_P3_MISTAKE_PLAIN_H
#define W14_P3_MISTAKE_PLAIN_H
int max_sessions = 8;
#endif
