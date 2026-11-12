int main(int argc, char**) {
  if (argc > 0) goto out;
  int code = 1;
out:
  return code - 1;
}
