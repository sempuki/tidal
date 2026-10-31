# A class that owns a resource

The task is to write a class that holds an array from `new[]` and
deletes the array exactly once. The class writes five functions: the
destructor, the copy constructor, the copy assignment operator, the move
constructor, and the move assignment operator.

The steps build one class, `Buffer`. Step 7 gives the whole class.

| Function | Job in `Buffer` | Rules |
|---|---|---|
| Constructor | Allocates the array and gives each element a value | {rule}`CTOR-6`, {rule}`NEW-2`, {rule}`NEW-3` |
| Destructor | Deletes the array | {rule}`DTOR-5`, {rule}`NEW-9` |
| Copy constructor | Allocates a new array and copies the elements | {rule}`CPY-1` |
| Copy assignment operator | Replaces the array with a copy of the other array | {rule}`CPY-3` |
| Move constructor | Takes the array from an rvalue | {rule}`CPY-2` |
| Move assignment operator | Deletes its own array and takes the other array | {rule}`CPY-3` |

## Steps

### 1. Allocate the array in the constructor

`new int[count]` makes an array whose size is known only at run time
({rule}`NEW-2`). The elements have no value after `new int[count]`. A
read before the first write is undefined ({rule}`NEW-3`), so the
constructor body writes each element.

The members are initialized in the order of their declarations, before
the body runs ({rule}`CTOR-6`). `explicit` stops an implicit conversion
from `int` to `Buffer` ({rule}`CTOR-9`).

```cpp
class Buffer {
  int size;
  int* data;
public:
  explicit Buffer(int count, int fill) : size(count), data(new int[count]) {
    for (int i = 0; i < size; ++i) data[i] = fill;
  }
  int count() const { return size; }
  int at(int i) const { return data[i]; }
};
```

### 2. Delete the array in the destructor

An object from `new` lives until a `delete` destroys the object. The end
of a scope has no effect on the object ({rule}`OBJ-5`). The destructor
of `Buffer` does that `delete`. These rules apply:

- An automatic `Buffer` is destroyed when control leaves its block, by
  any route ({rule}`OBJ-3`).
- The destructor body runs first. The members end after the body
  ({rule}`DTOR-5`).
- `delete[]` matches `new[]`. Plain `delete` on the array is undefined
  ({rule}`NEW-5`).
- The destructor has no `noexcept` specifier. The destructor is
  non-throwing, because no member of `Buffer` has a potentially
  throwing destructor ({rule}`DTOR-4`).

```cpp
  ~Buffer() { delete[] data; }       // in class Buffer
```

A user-declared destructor keeps the implicit copy functions
({rule}`CPY-4`). An implicit copy copies each member
({rule}`CPY-8`), so the copy gets the same pointer. Steps 3 and 4
replace those implicit functions. The first mistake in the Mistakes
section shows what happens without them.

### 3. Write the copy constructor

The copy constructor takes `const Buffer&` ({rule}`CPY-1`). The copy
constructor allocates its own array and copies each element. An lvalue
source uses the copy constructor ({rule}`CPY-9`).

```cpp
  Buffer(const Buffer& other) : size(other.size), data(new int[other.size]) {
    for (int i = 0; i < size; ++i) data[i] = other.data[i];
  }
```

A class whose objects must not be copied can delete the copy functions
instead. A use of a deleted function makes the program not valid
({rule}`DEL-1`).

```cpp
  Buffer(const Buffer&) = delete;
  Buffer& operator=(const Buffer&) = delete;
```

### 4. Write the copy assignment operator and test for self-assignment

The copy assignment operator is a member `operator=` that takes
`const Buffer&` ({rule}`CPY-3`). The operator deletes the old array,
allocates a new array, and copies the elements. The operator returns
`*this` by reference, as the implicit operator does ({rule}`CPY-6`).

The test `this != &other` handles `a = a`. In a self-assignment,
`other.data` and `data` are one pointer. Without the test, the copy
loop reads the new array, whose elements have no value
({rule}`NEW-3`).

```cpp
  Buffer& operator=(const Buffer& other) {
    if (this != &other) {
      delete[] data;
      size = other.size;
      data = new int[size];
      for (int i = 0; i < size; ++i) data[i] = other.data[i];
    }
    return *this;
  }
```

### 5. Write the move constructor

The move constructor takes `Buffer&&` ({rule}`CPY-2`). A non-`const`
rvalue source uses the move constructor ({rule}`CPY-9`). The move
constructor takes the pointer and sets the source pointer to null. The
destructor of the source then runs `delete[]` on a null pointer, which
does nothing ({rule}`NEW-4`).

A program makes an rvalue from a named object with a cast to an rvalue
reference. The cast gives an xvalue ({rule}`CAST-1`, {rule}`VC-3`).
A `return` of a local variable moves the variable without a cast
({rule}`CPY-11`).

```cpp
  Buffer(Buffer&& other) noexcept : size(other.size), data(other.data) {
    other.size = 0;
    other.data = nullptr;
  }
```

A function that the program writes is potentially throwing unless the
function has a `noexcept` specifier ({rule}`EXS-4`). The move functions
of `Buffer` have the specifier. If an exception leaves a `noexcept`
function, the program terminates ({rule}`UNW-5`).

### 6. Write the move assignment operator

The move assignment operator takes `Buffer&&` ({rule}`CPY-3`). The
operator deletes its own array, takes the source array, and leaves the
source empty. The test `this != &other` keeps a self-move from deleting
the array that the object keeps.

```cpp
  Buffer& operator=(Buffer&& other) noexcept {
    if (this != &other) {
      delete[] data;
      size = other.size;
      data = other.data;
      other.size = 0;
      other.data = nullptr;
    }
    return *this;
  }
```

The class declares a destructor and copy functions. So the class has
no implicit move functions, and without steps 5 and 6 an rvalue uses
the copy functions ({rule}`CPY-5`). A class that declares move
functions and no copy functions has deleted copy functions
({rule}`CPY-4`).

### 7. The whole class

```cpp
class Buffer {
  int size;
  int* data;
public:
  explicit Buffer(int count, int fill) : size(count), data(new int[count]) {
    for (int i = 0; i < size; ++i) data[i] = fill;
  }

  ~Buffer() { delete[] data; }

  Buffer(const Buffer& other) : size(other.size), data(new int[other.size]) {
    for (int i = 0; i < size; ++i) data[i] = other.data[i];
  }

  Buffer& operator=(const Buffer& other) {
    if (this != &other) {
      delete[] data;
      size = other.size;
      data = new int[size];
      for (int i = 0; i < size; ++i) data[i] = other.data[i];
    }
    return *this;
  }

  Buffer(Buffer&& other) noexcept : size(other.size), data(other.data) {
    other.size = 0;
    other.data = nullptr;
  }

  Buffer& operator=(Buffer&& other) noexcept {
    if (this != &other) {
      delete[] data;
      size = other.size;
      data = other.data;
      other.size = 0;
      other.data = nullptr;
    }
    return *this;
  }

  int count() const { return size; }
  int at(int i) const { return data[i]; }
};

Buffer make(int n) {
  Buffer result(n, 7);
  return result;                       // moved, or result is the return value
}

int main() {
  Buffer a(3, 1);
  Buffer b = a;                        // copy constructor
  Buffer c(1, 2);
  c = a;                               // copy assignment
  Buffer& same = c;
  c = same;                            // self-assignment: no change
  Buffer d = static_cast<Buffer&&>(a); // move constructor: a is now empty
  b = make(4);                         // move assignment
  Buffer& alias = b;
  b = static_cast<Buffer&&>(alias);    // self-move: no change
  if (a.count() != 0 || d.count() != 3 || c.at(2) != 1 || b.at(3) != 7) return 1;
  return 0;
}
```

The `return result;` either moves `result` or makes `result` the return
object. The choice is unspecified ({rule}`CPY-12`).

## Mistakes

A class with a destructor alone has a mistake. The implicit copy
constructor copies the pointer ({rule}`CPY-4`, {rule}`CPY-8`). The two
objects then delete one array two times ({rule}`NEW-5`).

```cpp
struct Leaky {
  int* data;
  explicit Leaky(int count) : data(new int[count]) {}
  ~Leaky() { delete[] data; }
};

int main() {
  Leaky a(4);
  Leaky b = a;     // implicit copy: b.data == a.data
}                  // undefined: the array is deleted two times
```

| Mistake | Example | Rule |
|---|---|---|
| Write the destructor and leave the implicit copy functions. | `Leaky b = a;` above | Undefined: a double delete ({rule}`NEW-5`) |
| Delete the array with plain `delete`. | `~Wrong() { delete data; }` | Undefined ({rule}`NEW-5`) |
| Leave out the self-assignment test in the copy assignment operator. | `a = a;` | Undefined: the copy reads elements that have no value ({rule}`NEW-3`) |
| Take the pointer in a move and leave the source pointer set. | `Owner(Owner&& o) : data(o.data) {}` | Undefined: both objects delete the array ({rule}`NEW-5`) |
| Use an element after its array is deleted. | `x = data[0];` after `delete[] data;` | Undefined ({rule}`OBJ-2`) |
| Read an element before the first write. | `new int[n]` and then `data[0]` | Undefined ({rule}`NEW-3`) |
| Declare only the move functions and then copy. | `Owner b = a;` | Not valid: the copy constructor is deleted ({rule}`CPY-4`, {rule}`CPY-21`) |
| Use a copy function that the class deletes. | `Buffer b = a;` with `= delete` | Not valid ({rule}`DEL-1`) |
| Throw from the destructor. | `~Buffer() { throw 1; }` | The program terminates ({rule}`DTOR-4`, {rule}`UNW-5`) |
