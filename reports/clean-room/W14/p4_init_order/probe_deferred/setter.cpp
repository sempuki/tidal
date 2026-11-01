// Nothing in main names anything from this unit. SINIT-5 lets the
// implementation defer this initialization until such a use, which never
// comes. The probe asks which choice the matrix compilers make.
extern int flag;
int set_flag() { flag = 1; return 1; }
int done = set_flag();
