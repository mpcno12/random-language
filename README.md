# random-language
A compiled WIP coding language.

# NOTICE
THIS IS A WORK IN PROGRESS! THERE ARE KNOWN ISSUES WITH TOKENIZATION ALONG WITH OTHER REGEX FEATURES. WE PLAN TO FIX THESE IN THE COMING TIME, BUT KNOW THAT NOTHING CURRENTLY WORKS, AND ALL FEATURES ARE SIMPLY WHAT WE PLAN ON IMPLEMENTING

## Features 
 1. AoT (Ahead of Time) Compilation
 2. Statically Typed Language
 3. Special Dynamically Typed Type
 4. Low Level Operations
 5. High Level Abstractions
 6. Case-insensitive Code Parsing

### Features - Explained
 1. AoT (Ahead of Time) Compilation:
    > AoT (Ahead of Time) Compilation is a method of *compiling* in which the source code is compiled *Ahead of Time*, meaning that you could compile your code on your device, and any User could use it as long as it supports their hardware & software. 
 2. Statically Typed Language:
    > Statically Typed Languages are Languages in which the type of a variable is known & checked at Compile Time. This certainly makes the language harder to write, but many issues that come with *Dynamically Typed Languages* no longer apply.
 3. Special Dynamically Typed Type:   
    > Our language is a *Statically Typed Language*, which gets rid of many errors, but has added complexity to it. We offer a special type that automatically comforms to the type assigned to it. 
    This does have some issues with this. This type is useless for *Immutable Variables*, or unchanging variables, in which you know their type ahead of time. However, for mutable variables or inputs to a function which can accept anything, this is the perfect type to use. 
 4. Low Level Operations:
    > Low Level Operations allows you closer access to the underlying hardware itself. You can manipulate Memory, directly allocate items, manage pointers, and other things a Higher Level Language won't let you / abstract away from you.
 5. High Level Abstractions:
    > Now, Low Level Operations isn't to the appeal of many. It's easy to shoot yourself in the foot if you do something wrong. Unlike Low Level Operations, High Level Abstractions get's rid of the ability to shoot yourself in the foot. This, in turn, gets rid of your ability to shoot in the first place. Sure you can take down an issue, but not with the same efficiency. This, in turn, decreases the learning curve by quite a bit.
 6. Case-insensitive Code Parsing:
    > Many issues with modern day programming languages is that they are Case-sensitive. You turn a lowercase c to a capital C and suddenly the compiler screams at you, or worse, your runtime silently crashes on you. Unlike those languages, this programming language is Case-insensitive, thus, even if you write "interface" like "InTerFAce", it still has the same effect as any other styles of writing. As long as it's still the same word.      

## Keywords (And definitions)
1. let: declares an immutable variable.
2. mut: declares a variable to be mutable.
3. func: defines a function
4. if: an if statement executing code based on if the statement it checks is true.
5. else: an else statement executes code when it's paired if statement find's it's statement to be false.
6. while: a while statement is similar to an if statement, but it executes it's code continously until it's statement is false.
7. public: declares an item to be usable by all.
8. interface: declares an item that can be inherited by other items (similar to Rust's trait keyword)
9. private: declares an item to be usable exclusively by items within it's own scope.
10. null: A statement which is equal to Nothing.
11. nullptr: A statement which is a pointer pointing to a value equal to Nothing.
12. return: returns out of a function.


## Features Done being Made:
[] - AoT Compilation
[] - Statically Typed Language
[] - Special Dynamically Typed Type
[] - Low Level Operation
[] - High Level Abstractions
[x] - Case-insensitive Writing