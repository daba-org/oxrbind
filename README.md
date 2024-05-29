# Table of Content

[toc]

# Overview

*oxrbind* is a safe binding for the *OpenXR SDK*.
It is intended to be used both by application developers to interface with an OpenXR compliant runtime,
and by runtime/API layer developers to create OpenXR compliant runtime/API extension.

# Clone this repo

Since this repo uses git submodule to manage *OpenXR SDK* it must be cloned like this:

```bash
git clone --recurse-submodules ${url}
```

If you have already cloned the repo without the submodule, you can run this command to init it:

```bash
git submodule update --init
```

# Build Dependencies

These are the non-Rust dependencies required to compile oxrbind, mostly because we need to compile the OpenXR SDK.

- C++ compiler (MSVC, g++, etc.)
- CMake (`cmake` must exist on `PATH`)
- LLVM (Must define the `LIBCLANG_PATH` environment variable to be `${LLVM_HOME}/lib`. This is required by `bindgen`.)
