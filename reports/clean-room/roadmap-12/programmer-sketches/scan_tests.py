#!/usr/bin/env python3
"""Read-only scans of tests/ for the forms of this round's reductions.
Prints the count and up to eight files for each form. The patterns are
rough; read the hits."""
import re, glob, os
root = os.path.join(os.path.dirname(os.path.abspath(__file__)), '..', '..', '..', '..', 'tests')
files = {f: open(f, errors='ignore').read() for f in glob.glob(root + '/**/*.cpp', recursive=True)}
def report(name, pred):
    hits = [os.path.relpath(f, root) for f, s in files.items() if pred(s)]
    print(f'{name}: {len(hits)}'); [print('  ' + h) for h in hits[:8]]

# 1. friend class X; inside a class nested in X.
def friend_enclosing(s):
    for m in re.finditer(r'(?:class|struct)\s+(\w+)\s*(?:final\s*)?(?::[^{]*)?\{', s):
        body = s[m.end():m.end() + 1500]
        if re.search(r'(?:class|struct)\s+\w+\s*\{[^}]*friend\s+(?:class|struct)\s+' + m.group(1) + r'\s*;', body):
            return True
    return False
report('friend class Enclosing; in a nested class', friend_enclosing)

# 2. A literal operator template with a char pack.
report('literal operator template', lambda s: re.search(r'template\s*<\s*char\s*\.\.\.\s*\w*\s*>\s*[^;{]*operator\s*""', s) is not None)

# 3. A raw literal operator used with a literal of 20 or more digits.
report('raw literal operator with a long literal',
       lambda s: re.search(r'operator\s*""\s*\w+\s*\(\s*const char\s*\*\s*\w*\s*\)', s) is not None and re.search(r'\b\d{20,}_\w+', s) is not None)

# 4. A lambda with a trailing return type that names a template-id and a braced return.
report('lambda -> Name<...> with return {...}',
       lambda s: re.search(r'\]\s*\([^)]*\)\s*(?:mutable\s*)?->\s*\w+\s*<[^>]*>\s*\{[^}]*return\s*\{', s) is not None)

# 5. A class with a non-template constructor and a constructor template,
#    list-initialized with one argument (rough).
def braced_beside_template(s):
    for m in re.finditer(r'(?:struct|class)\s+(\w+)\s*\{', s):
        n = m.group(1); body = s[m.end():m.end() + 800]
        if re.search(r'\n\s*' + n + r'\s*\(', body) and re.search(r'template\s*<[^>]*>\s*(?:explicit\s*)?' + n + r'\s*\(', body):
            if re.search(r'\b' + n + r'\s+\w+\s*\{[^,}]+\}', s):
                return True
    return False
report('class with a constructor and a constructor template, braced with one argument', braced_beside_template)

# 6. Allocated storage held as unsigned char*, cast to a class pointer, then placement new.
report('unsigned char* from operator new, then static_cast to T* and placement new',
       lambda s: re.search(r'unsigned char\s*\*\s*\w+\s*=\s*static_cast<unsigned char\s*\*>\(\s*::operator new', s) is not None and 'new (' in s.replace('new(', 'new ('))

# 7. A constant template parameter whose type is a template-id.
report('constant template parameter of type Name<...>',
       lambda s: re.search(r'template\s*<\s*\w+\s*<[^<>]+>\s+\w+\s*>', s) is not None)
