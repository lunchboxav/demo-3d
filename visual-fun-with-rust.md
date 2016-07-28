# Visual Fun with Rust

## Adityo Pratomo (@kotakmakan)

---

# Background

- Chief Academic Officer at Framework
- Chief Technology OFficer at Labtek Indie

----

- Generalist
- Creative Coder
- C/C++, Java, JS

---

# Me and C++

- Met during undergrad (2005)
- Professionally using it since 2012
- Tool: openFrameworks, Cinder and Unreal Engine

----

- Recent work: Interactive Video and Sound for Australia National Maritime Museum
- Coded in C++, running in Raspberry Pi

----

- (+) Fast product
- (+) Runs on many platforms
- (+) Robust debugging tool
- (-) Still have issues with pointers
- (-) Segfaults here and there

---

# When I Meet Rust

- System programming language
- Like C++, without segfaults (yum!)
- Better handling of reference and pointers
- Mixture of imperative and functional paradigm

---

# What to Do with Rust?

- System programming
- Something low-level enough to benefit from precise memory management
    - Web Browser (Mozilla Firefox)
    - Distributed storage system (Dropbox)
    - 3D Games

---

# Learning Rust by Doing

- Context for personal learning -> doing something exciting and familiar enough
- Learn guitar by playing song, not by memorizing chords

---

# Learning Rust the Visual Way

- Project: exploring 3D programming in Rust
- Make 3D scene using Rust
- Do low-level OpenGL calls
- Interop with standard C shaders
- More tangible result

---

# Glium

- One of many OpenGL-related Crates in Rust
- Allows for doing OpenGL calls, Rust style, while still writing shaders in C
- Pretty popular and complete

---

# Drawing Shape

- Let's make a triangle
- Rust can talk to C shaders
- Vertex shader determines the position of the shape
- Fragment shader determines the color of the shape

---

# Animating Shape

- We can use uniforms to dynamically change the shape coordinate over time
- Uniform can be a 4x4 transform matrix

---

# Color Triangle

- We can also make Vertex and Fragment shader talk to each other
- The color of each pixel is determined by its position

---

# Texturing

- Texturing means putting image on top of a shape
- Start by importing image
- Now, we add the texture data to determine which part of the image should be clamped on which part of the shape
- The transformation matrix is still valid

---

# Loading 3D Object

- Load vertices, normals and indices of a complex shape, from an additional file
- Then drawing using the previous method

---

# Shade 3D Object

- To make it look 3D, we can implement Goraud Shading, the most basic Shading
- This way, we can make it looks glossier and have depth
- Achieved by counting the normal vectors for each vertices in the shape and multiplying it with a certain light vectors

---

# What I've Learned So Far

---

# 1. Low Level System Programming

---

# 2. Interopability with Other

- Rust can talk to C shader
- Also, can talk to python, node.js, Ruby, others via FFI
- Use it like Dropbox, to optimize certain part of the

---

# 3. Statement and Expressions

- Statement -> executing code
- Expression -> returning a value
- Looks a bit like functional programming

```
fn sqr(i: i32) -> i32 { i * i }
fn abs(i: i32) -> i32 { if i >= 0 { i } else { -i } }

if t > 0.5 { t = -0.5; }

```

---

# 4. Pattern Matching

- More powerful dan traditional if-else
- Must caters to the whole scenario, to provide basic error handling

```
match ev {
    glium::glutin::Event::Closed => return,
    _ => ()
}

```

---

# 5. Referencing

---

# 6. Variables

---

# 7. Compiler

---

# 8. Modularity

---

# 9. Crate

---

# 10. Macro

---

# Learning Tips
