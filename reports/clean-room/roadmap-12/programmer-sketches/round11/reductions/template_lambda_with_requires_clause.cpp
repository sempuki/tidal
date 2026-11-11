// A lambda with a template head and a trailing requires clause. From
// one-line/template_lambda.cpp.
int main() {
  auto f = []<class T>(T x) requires (sizeof(T) > 1) { return x; };
  return f(0);
}
