![Elitebuild :)](resources/Elitebuild_Banner.png)

# Fegeya Elitebuild

## Small, powerful, work-in-progress build system. Written in Rust.

### Features:
 * No functions (all are built-ins)
 * All variables are global
 * Cross-platform (say 'thank you' to rust's standard lib)
 * Different syntax.
 * Preprocessor.
 * Aliases.

### A taste of Elite's syntax:
```cpp
set BIN_PATH      as "/usr/bin/"
set COMPILER      as "g++"
set COMPILER_PATH as "{BIN_PATH}{COMPILER}"

set SOURCE_FILE   as "example.cpp"
set OUTPUT        as "example"

for signal "start" [
  for exists "{BIN_PATH}clang++" [
    set COMPILER as "clang++"    
  ]
  
  for specific "linux" [
    println "OS: GNU/Linux"
  ]
  
  for specific "freebsd" [
    println "OS: FreeBSD"
  ]
  
  for specific "windows" [
    println "OS: Windows"
  ]
  
  for specific "openbsd" [
    println "OS: OpenBSD"
  ]
  
  for argument "build" [
    use exec "{COMPILER} {SOURCE_FILE} -o {OUTPUT}"
  
    for exists $OUTPUT [
      println "Build succeeded"
    ]
    
    use signal "exit"
  ]
   
  use signal "exit"
]
```

### Other implementations?
  * [For C++ as ElitedotC++](https://github.com/ferhatgec/elite.cpp)

### Elitebuild licensed under the terms of MIT License.
