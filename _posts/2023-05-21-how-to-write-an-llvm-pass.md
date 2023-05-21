---
# layout: post
title: "How to write an LLVM Pass"
date: 2023-05-21
---

The official tutorial of writing an LLVM Pass is [here](https://www.llvm.org/docs/WritingAnLLVMPass.html). 

This post is mainly about writing an LLVM Pass in a standalone style and using the compiled LLVM Pass (the _.so_  file) in the compilation time (_e.g._, compile a target of instrumentation with clang).

## 0. Prerequisites

- Ubuntu (here we take Ubuntu 20.04 as a example)
- LLVM (default version 10) 
- clang (default version 10)

Install LLVM and clang of default version using the following command:
```bash
sudo apt install llvm clang
```

## 1. Create files to contain an LLVM Pass

Assuming that we are under a user directory (_i.e._, `cd ~`), we create a new directory named `llvm-pass` and enter it:
```bash
mkdir llvm-pass && cd llvm-pass
```

Then, we create a folder to contain our custom LLVM Pass named `Hello`:
```bash
mkdir Hello && cd Hello
```

In the `Hello` folder, we `touch` a new `Hello.cpp` and edit it.

## 2. Write an LLVM Pass

First, we add those "include" statements at the start of the `Hello.cpp`:
```cpp
#include "llvm/Pass.h"
#include "llvm/IR/Function.h"
#include "llvm/Support/raw_ostream.h"
```

Second, we add a `using namespace` statement to bring things needed into scope:
```cpp
using namespace llvm;
```

Then, we write our LLVM Pass which is a subclass of `FunctionPass` in an anonymous namespace:
```cpp
namespace {
    struct Hello : public FunctionPass {
        static char ID;  //  used by LLVM to identify pass
        Hello() : FunctionPass(ID) {}

        bool runOnFunction(Function &F) override {
            errs() << "Hello: ";
            errs().write_escaped(F.getName()) << '\n';
            return false;
        }
    };
}
```
The function `runOnFunction`, which overrides an abstract virtual method inherited from `FunctionPass`, in the pass will be executed on a function at a time. In this example, it means that the string comprised of "Hello: " and the function name is printed to `stderr` everytime a function is encountered.

To finish writing this pass, we initialize pass ID and register our pass class `Hello` at the end of `Hello.cpp`:
```cpp
char Hello::ID = 0;
static RegisterPass<Hello> X(
    "hello",        // the name of command line argument
    "Hello Pass",   // pass name
    false,          //  if a pass walks CFG without modifying it then the third argument is set to true
    false           // if a pass is an analysis pass, for example dominator tree pass, then true is supplied
);
```

## 3. Build the LLVM Pass

The following command is used to build the custom LLVM Pass we write to a shared library (_.so_ file):
```bash
clang++ -shared -fPIC -o <output-so-file-name>.so <your-pass-name>.cpp $(llvm-config --cxxflags --ldflags --libs)
```

In this example, we simply replace those two placeholder with "Hello":
```bash
clang++ -shared -fPIC -o Hello.so Hello.cpp $(llvm-config --cxxflags --ldflags --libs)
```

Then, you will see the `Hello.so` under `Hello` directory.

## 4. Use the LLVM Pass

To use the LLVM Pass just written by us, there are two ways: (1) load the pass at the optimization pipeline (_e.g._, use llvm optimization tool `opt`), and (2) load the pass with a compiler (_e.g._, `clang`).

Here, we use a simple C file to demonstrate the two ways.
```c
/* main.c */
#include <stdio.h>

int hello() {
    return 0;
}

int main() {
    hello();
    return 0;
}
```

### 4.1 Use a pass at LLVM optimization phase

To load the pass, we need to get the LLVM bitcode file of this C code:
```bash
clang -emit-llvm -c main.c -o main.bc
```

Then load and enable our Hello pass with `opt`:
```bash
opt -load path-to-pass/Hello.so -hello < main.bc > /dev/null
```
In this example, we just discard the optimized bitcode file by redirecting the `stdout` to `/dev/null`. If the pass modifies the code (__e.g.__, adding or deleting some instructions), you can save the optimized bitcode by redirecting the `stdout` to some file. For example, we can save the output above using `> main_opt.bc`.

After the pass, information or data collected during the pass will print according to your setting. If you use the pass to modify the original code, the optimized bitcode file (_e.g._, `main_opt.bc`) can be compiled to an executable and those code modifications will take effect when running that executable. For instance:
```bash
clang main.bc -o main
```

### 4.2 Use a pass with a compiler

Using passes at optimization phase sometimes needs more effort and is tedious, because you always need to get the LLVM bitcode (.bc files) of the target and run `opt` to enable passes. 

There is a approach that can use passes just as the command line argument of a compiler, which make pass loading more convenient and without complicated process.

First, we need to add following line at the end of `Hello.cpp` and rebuild it:
```cpp
static void registerHello(const PassManagerBuilder &, llvm::legacy::PassManagerBase &PM) {
    PM.add(new Hello());
}

static RegisterStandardPasses RegisterHello(PassManagerBuilder::EP_EarlyAsPossible, registerHello);
```

Now, you can use a simpler command to compile the target with your pass loaded and enabled.
```bash
clang -Xclang -load -Xclang path-to-pass/Hello.so <your-source-files>
```

## Reference

- [Official tutorial of writing an LLVM Pass](https://www.llvm.org/docs/WritingAnLLVMPass.html)
- [Run an LLVM Pass Automatically with Clang](https://www.cs.cornell.edu/~asampson/blog/clangpass.html).