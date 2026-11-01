extern int flag;
int set_flag() {
  flag = 1;
  return 1;
}
int done = set_flag();
