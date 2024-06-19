# Rust DLL Injection

A simple example of compiling both a DLL in Rust a means of injecting it.

The lib.rs is a simple DLL that will allocate a new console to a process and print a message to it.

The main.rs is used to perform DLL injection via the dll_syringe crate.
