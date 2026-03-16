-   [Home](https://www.agner.org/)
-   [Optimization manuals](#manuals)

    ::: dropcontent
    [1. Optimizing software in C++](#manual_cpp) [2. Optimizing subroutines in assembly language](#manual_asm) [3. The microarchitecture of Intel, AMD and VIA CPUs](#manual_microarch) [4. Instruction tables](#manual_instr_tab) [5. Calling conventions](#manual_call_conv)
    :::
-   [Vector class library](#vectorclass)
-   [ForwardCom](#forwardcom)
-   [Software](#)

    ::: dropcontent
    [Test program for measuring clock cycles](#testp) [Object file converter and disassembler](#objconv) [Assembly function library](#asmlib)
    :::
-   [Discussions](#)

    ::: dropcontent
    [Floating point exception tracking and NaN propagation](#nan_propagation) [ForwardCom instruction set](#forwardcom) [My blog on optimization](https://www.agner.org/forum/viewforum.php?f=1)
    :::
-   [Links](#links)

    ::: dropcontent
    [Software optimization links](#links) [My blog on optimization](https://www.agner.org/forum/viewforum.php?f=1)
    :::

# Software optimization resources

## Contents

-   [Optimization manuals](#manuals)
-   [Vector class library](#vectorclass)
-   [ForwardCom: An open standard instruction set for high performance microprocessors](#forwardcom)
-   [Test programs for measuring clock cycles in C++ and assembly code](#testp)
-   [Object file converter and disassembler](#objconv)
-   [Assembly function library](#asmlib)
-   [Floating point exception tracking and NaN propagation](#nan_propagation)
-   [CPUID manipulation program](#cpuidfake)
-   [Links](#links)
-   [My blog on optimization](https://www.agner.org/forum/viewforum.php?f=1)

------------------------------------------------------------------------

[]{#manuals}

## Optimization manuals

This series of five manuals describes everything you need to know about optimizing code for x86 and x86-64 family microprocessors, including optimization advices for C++ and assembly language, details about the microarchitecture and instruction timings of most Intel, AMD and VIA processors, and details about different compilers and calling conventions.

Operating systems covered: DOS, Windows, Linux, BSD, Mac OS X Intel based, 32 and 64 bits.

Note that these manuals are not for beginners.

[]{#manual_cpp}1. Optimizing software in C++: An optimization guide for Windows, Linux and Mac platforms
:   This is an optimization manual for advanced C++ programmers. Topics include: The choice of platform and operating system. Choice of compiler and framework. Finding performance bottlenecks. The efficiency of different C++ constructs. Multi-core systems. Parallelization with vector operations. CPU dispatching. Efficient container class templates. Etc.\
     \
    [File name]{.underline}: optimizing_cpp.pdf, size: 1848238, last modified: 2025-Dec-15.\
    [Download](optimizing_cpp.pdf).\
     

[]{#manual_asm}2. Optimizing subroutines in assembly language: An optimization guide for x86 platforms
:   This is an optimization manual for advanced assembly language programmers and compiler makers. Topics include: C++ instrinsic functions, inline assembly and stand-alone assembly. Linking optimized assembly subroutines into high level language programs. Making subroutine libraries compatible with multiple compilers and operating systems. Optimizing for speed or size. Memory access. Loops. Vector programming (XMM, YMM, SIMD). CPU-specific optimization and CPU dispatching.\
     \
    [File name]{.underline}: optimizing_assembly.pdf, size: 1091275, last modified: 2025-Dec-18.\
    [Download](optimizing_assembly.pdf).\
     

[]{#manual_microarch}3. The microarchitecture of Intel, AMD and VIA CPUs: An optimization guide for assembly programmers and compiler makers
:   This manual contains details about the internal working of various microprocessors from Intel, AMD and VIA. Topics include: Out-of-order execution, register renaming, pipeline structure, execution unit organization and branch prediction algorithms for each type of microprocessor. Describes many details that cannot be found in manuals from microprocessor vendors or anywhere else. The information is based on my own research and measurements rather than on official sources. This information will be useful to programmers who want to make CPU-specific optimizations as well as to compiler makers and students of microarchitecture.\
     \
    [File name]{.underline}: microarchitecture.pdf, size: 2578972, last modified: 2025-Dec-15.\
    [Download](microarchitecture.pdf).\
     

[]{#manual_instr_tab}4. Instruction tables: Lists of instruction latencies, throughputs and micro-operation breakdowns for Intel, AMD and VIA CPUs
:   Contains detailed lists of instruction latencies, execution unit throughputs, micro-operation breakdown and other details for all common application instructions of most microprocessors from Intel, AMD and VIA. Intended as an appendix to the preceding manuals. Available as pdf file and as spreadsheet (ods format).\
     \
    [File name]{.underline}: instruction_tables.pdf, size: 2248996, last modified: 2025-Sep-20.\
    [Download](instruction_tables.pdf).\
     \
    [File name]{.underline}: instruction_tables.ods, size: 557154, last modified: 2025-Sep-20.\
    [Download](instruction_tables.ods).\
     

[]{#manual_call_conv}5. Calling conventions for different C++ compilers and operating systems
:   This document contains details about data representation, function calling conventions, register usage conventions, name mangling schemes, etc. for many different C++ compilers and operating systems. Discusses compatibilities and incompatibilities between different C++ compilers. Includes information that is not covered by the official Application Binary Interface standards (ABI\'s). The information provided here is based on my own research and therefore descriptive rather than normative. Intended as a source of reference for programmers who want to make function libraries compatible with multiple compilers or operating systems and for makers of compilers and other development tools who want their tools to be compatible with existing tools.\
     \
    [File name]{.underline}: calling_conventions.pdf, size: 1078737, last modified: 2023-Jul-01.\
    [Download](calling_conventions.pdf).\
     

All five manuals
:   Download all the above manuals together in one zip file.\
     \
    [File name]{.underline}: optimization_manuals.zip, size: 7400333, last modified: 2025-Dec-18.\
    [Download](optimization_manuals.zip).\
     

------------------------------------------------------------------------

## [C++ vector class library]{#vectorclass}

This is a collection of C++ classes, functions and operators that makes it easier to use the the vector instructions (Single Instruction Multiple Data instructions) of modern CPUs without using assembly language. Supports the SSE2, SSE3, SSSE3, SSE4.1, SSE4.2, AVX, AVX2, FMA, XOP, and AVX512F/BW/DQ/VL instruction sets. Includes standard mathematical functions. Can compile for different instruction sets from the same source code.\
[Description and instructions](https://www.agner.org/optimize/vcl_manual.pdf).\
[Message board](https://www.agner.org/forum/viewforum.php?f=1).\
[Source on Github](https://github.com/vectorclass).\
[Nice little instruction video by WhatsaCreel](https://www.youtube.com/watch?v=TKjYdLIMTrI).

[Latest release](https://github.com/vectorclass/version2/releases).\

------------------------------------------------------------------------

## [ForwardCom: An open standard instruction set for high performance microprocessors]{#forwardcom}

This is a proposal and discussion of how an ideal instruction set architecture can be constructed. The proposed instruction set combines the best from the RISC and CISC principles to produce a flexible, consistent, modular, orthogonal, scalable and expansible instruction set for high performance microprocessors and large vector processors.

The ForwardCom instruction set has variable-length vector registers and a special addressing mode that allows the software to automatically adapt to different microprocessors with different maximum vector lengths and make efficient loops through arrays regardless of whether the array size is divisible by the vector length. Standardization of the corresponding ecosystem of ABI standards, function libraries, compilers, etc. makes it possible to combine different programming languages in the same program.

Development tools and softcore are available.

Introduction: [www.forwardcom.info](https://www.forwardcom.info/).\
[ForwardCom manual](https://github.com/ForwardCom/manual/raw/master/forwardcom.pdf).\

------------------------------------------------------------------------

## [Test programs for measuring clock cycles and performance monitoring]{#testp}

Test programs that I have used for my research. Can measure clock cycles and performance monitor counters such as cache misses, branch mispredictions, resource stalls etc. in a small piece of code in C, C++ or assembly. Can also set up performance monitor counters for reading inside another program. Supports Windows and Linux, 32 and 64 bit mode, multiple threads.

For experts only. Useful for analyzing small pieces of code but not for profiling a whole program.

[File name]{.underline}: testp.zip, size: 1008078, last modified: 2025-Aug-28.\
[Download](testp.zip).

------------------------------------------------------------------------

## [Object file converter]{#objconv}

This utility can be used for converting object files between COFF/PE, OMF, ELF and Mach-O formats for all 32-bit and 64-bit x86 platforms. Can modify symbol names in object files. Can build, modify and convert function libraries across platforms. Can dump object files and executable files. Also includes a very good disassembler supporting the SSE4, AVX, AVX2, AVX512, FMA3, FMA4, XOP and Knights Corner instruction sets. Source code included (GPL). [Manual](objconv-instructions.pdf).

[File name]{.underline}: objconv.zip, size: 1080800, last modified: 2025-Oct-30.\
[Download](objconv.zip).

------------------------------------------------------------------------

## [Assembly function library]{#asmlib}

This is a library of optimized subroutines coded in assembly language. The functions in this library can be called from C, C++ and other compiled high-level languages. Supports many different compilers under Windows, Linux, BSD and Mac OS X operating systems, 32 and 64 bits. This library contains faster versions of common C/C++ memory and string functions, fast functions for string search and string parsing, fast integer division and integer vector division, as well as several useful functions not found elsewhere.

The package contains library files in many different file formats, C++ header file and assembly language source code. Gnu general public license applies. [Manual](asmlib-instructions.pdf).

[File name]{.underline}: asmlib.zip, size: 794549, last modified: 2023-May-03.\
[Download](asmlib.zip).

------------------------------------------------------------------------

## [Floating point exception tracking and NaN propagation]{#nan_propagation}

This article discusses the problems with floating point exceptions in systems that use out-of-order execution and SIMD parallelism. A solution based on NaN propagation is recommended.\
 \
[File name]{.underline}: nan_propagation.pdf, size: 319072, last modified: 2026-Feb-27.\
[Download](nan_propagation.pdf).\
 

------------------------------------------------------------------------

## [CPUID manipulation program for VIA]{#cpuidfake}

This is a program that can change the CPUID vendor string, family and model number on VIA Nano processors. See [my blog](http://www.agner.org/optimize/blog/read.php?i=49#73) for a discussion of the purpose of this program.

[File name]{.underline}: cpuidfake.zip, size: 67593, last modified: 2010-Aug-08.\
[Download](cpuidfake.zip).

------------------------------------------------------------------------

[]{#links}

## Useful software optimization links

Agner\'s CPU blog [www.agner.org/forum](https://www.agner.org/forum/)

CPU-id tools and information [www.cpuid.com](https://www.cpuid.com)

Godbolt compiler explorer. This is a very useful online tool to test how different compilers treat a piece of code. Supports the vector class library. [www.godbolt.org](https://www.godbolt.org/)

Masm Forum [www.masmforum.com](https://www.masmforum.com/)

ASM Community Messageboard [www.asmcommunity.net/forums](https://www.asmcommunity.net/forums/)

Hutch\'s masm pages [www.masm32.com](https://www.masm32.com/)

likwid performance measuring tools for Linux [github.com/RRZE-HPC/likwid](https://github.com/RRZE-HPC/likwid)

Bit Twiddling Hacks [graphics.stanford.edu/\~seander/bithacks.html](https://graphics.stanford.edu/~seander/bithacks.html)

Programmer\'s heaven assembler zone [Programmers\' Heaven](https://programmersheaven.com/categories/x86_asm)

Virtual sandpile x86 Processor information [www.sandpile.org](https://www.sandpile.org)

Online computer books [www.computer-books.us/assembler.php](http://www.computer-books.us/assembler.php)

Instruction latency listings [instlatx64.atw.hu/](http://instlatx64.atw.hu/) and [uops.info](https://uops.info/)

NASM assembler [www.nasm.us/](https://www.nasm.us/)

FASM assembler and messageboard [flatassembler.net](https://flatassembler.net/)

JWASM assembler [www.japheth.de](https://www.japheth.de/)

Yeppp open source library of assembly language functions [bitbucket.org/MDukhan/yeppp](https://bitbucket.org/MDukhan/yeppp/src/default/)

MAQAO (Modular Assembly Quality Analyzer and Optimizer), a tool for analyzing and optimizing binary codes. [www.maqao.org](http://www.maqao.org/)

[Newsgroup: comp.lang.asm.x86](https://groups.google.com/forum/#!forum/comp.lang.asm.x86)

### Intel resources

Reference manuals and other documents can be found at Intel\'s web site. Intel\'s web site is refurnished so often that any link I could provide here to specific documents would be broken after a few months. I will therefore recommend that you use the search facilities at [www.intel.com](https://www.intel.com/content/www/us/en/design/resource-design-center.html) and search for \"Software Developer\'s Manual\" and \"Optimization Reference Manual\".

### AMD resources

[www.amd.com/en/search/documentation/hub.html](https://www.amd.com/en/search/documentation/hub.html)

### Microsoft resources

MASM manuals [Microsoft Macro Assembler reference](https://docs.microsoft.com/en-us/cpp/assembler/masm/microsoft-macro-assembler-reference?view=vs-2019)

3919839

-   [Home](https://www.agner.org/)
-   [Optimization manuals](#manuals)

    ::: dropcontent
    [1. Optimizing software in C++](#manual_cpp) [2. Optimizing subroutines in assembly language](#manual_asm) [3. The microarchitecture of Intel, AMD and VIA CPUs](#manual_microarch) [4. Instruction tables](#manual_instr_tab) [5. Calling conventions](#manual_call_conv)
    :::
-   [Vector class library](#vectorclass)
-   [ForwardCom](#forwardcom)
-   [Software](#)

    ::: dropcontent
    [Test program for measuring clock cycles](#testp) [Object file converter and disassembler](#objconv) [Assembly function library](#asmlib)
    :::
-   [Discussions](#)

    ::: dropcontent
    [Floating point exception tracking and NaN propagation](#nan_propagation) [ForwardCom instruction set](#forwardcom) [My blog on optimization](https://www.agner.org/forum/viewforum.php?f=1)
    :::
-   [Links](#links)

    ::: dropcontent
    [Software optimization links](#links) [My blog on optimization](https://www.agner.org/forum/viewforum.php?f=1)
    :::
