
Usage
=====

The compiler is still a work-in-progress, and is not yet
functional. Once it is functional, however, it can be invoked using:

    rygg vmu compile example.rg -output example.vms

To assemble a file, use the `rygg vmu assemble` command:

    rygg vmu assemble example.s -output example.vms

For help, run `rygg help`:

    % rygg help
    Compiler and build tool for Dreamcast VMU
    
      rygg SUBCOMMAND
    
    === subcommands ===
    
      vmu      Operations for Dreamcast VMU
      version  print version information
      help     explain a given subcommand (perhaps recursively)

The `help` command can be used on subcommands as well:

    % rygg help vmu
    Operations for Dreamcast VMU
    
      rygg vmu SUBCOMMAND
    
    === subcommands ===
    
      assemble  Assembler for Dreamcast VMU
      compile   Compiler for the Rygg programming language
      help      explain a given subcommand (perhaps recursively)

---

    % rygg vmu help assemble
    Assembler for Dreamcast VMU
    
      rygg vmu assemble INPUT-FILE
    
    === flags ===
    
      -output  output-file
               (alias: -o)
      [-help]  print this help text and exit
               (alias: -?)

Build
=====

Requires opam 2.0.

When trying to build for the first time, you may want to create a
local switch. To do so, `cd` into the source directory, and run:

    opam switch create . --empty

Install dependencies:

    opam install . --deps-only --unlock-base --locked -y

To build using dune directly, run

    dune build @all

To build with all warnings as errors:

    dune build @all --profile check

To build a statically-linked executable (doesn't work on macOS):

    dune build @all --profile static

Lockfile
========

To re-generate the lockfile, run `opam lock .`.

Unit Tests
==========

To run the unit tests, use

    dune runtest

utop
====

Ensure `utop` is installed:

    opam install utop

Start `utop` with the sources from the current project:

    dune utop src

Links
=====

([How to do local builds](https://opam.ocaml.org/blog/opam-install-dir/))

([Lock files in opam](https://opam.ocaml.org/blog/opam-20-tips/))
