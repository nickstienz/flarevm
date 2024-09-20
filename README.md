# Flare VM (FVM)

> I'm switching from helix to neovim, this might take a while...
> This project is being rewritten from scratch ;-;

A VM written in Rust designed to teach low-level programming 
and the internals of a computer.

The VM will use a custom networking system in order to send commands and state
between the client and server (VM). The project will have a built-in cli client
so you don't need to worry about the client-server fun but being able to extend
the functionality of the VM by creating graphical representation of things and
such will be possible using this approach. You could also have multiple clients
connected to the VM which may be good in a learning environment.

This VM will try to replicate an x86_64 as best as it can meaning the internal
sturcture and runtime will be as close to that as possible. Some parts may need
to be abstracted but will have warnings about that and the differences.
