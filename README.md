# Intro To Rust And It's OOP Characteristics

I will be giving a brief introduction to Rust and it's syntax from its Primitive DataTypes to its built in Data Structures such as Vectors, Arrays, and HashMaps. Throughout the introduction I will include similar declarations and definitions from C and Java to help compare and contrast the key features that Rust offers.

After reading this paper, it is my goal that you will have attained the following:

###### - A basic understanding of the functionality of Rust and the power it provides through its safety and concurrency
###### - A better understanding of what makes object oriented languages object oriented
###### - A new found enjoyment for what I like to call barebones programming

## Let's Start With, What Is Rust?

Rust is a systems programming language that is designed for performance, safety, and concurrency. It is a strongly-typed language, meaning that every value and expression in Rust has a specific data type that the compiler checks at compile-time to ensure that the program is correct.

In Rust, variables must be declared before they can be used. This means specifying both the name of the variable and its data type. For example, to declare an unsigned 16-bit integer variable in Rust, you would write:

```Rust
let my_unsigned_int: u16 = 42;
```

Here, `my_unsigned_int` is the name of the decalred variable, `u16` is the DataType and size, and `42` is the initalized value. In Rust, variable names are written in snake_case, with lowercase words separated by underscores, in contrast to most OOP languages such as C and Java where camel case is encouraged. 

###### Note: In Rust, the need to specify variable size, i.e u16, is not always needed as Rusts compiler has dynamic typing to some extent. For example

```Rust
let my_unsigned_int = 42
```

This assignment statement would work without errors due to Rusts dynamic typing. However, assigning a DataType such as in the prior example will enforce all values assigned to the variable to be of that type, where as the latter example can take just about any type at initalization. In this case, after initalization, the type of the value assigned to the variable is binded to the variable and it is unable to change its type without an explicit `let` declaration again.

In Rust, variables can be declared as either mutable or immutable. Immutable variables cannot be changed after they are initialized, while mutable variables can be modified. To declare a mutable variable in Rust, you use the mut keyword, like so:

```Rust
let mut my_var: u16 = 42;
```

### Primitive DataTypes

Rust has several primitive data types, including integers, floating-point numbers, booleans, characters, and arrays. The integer types in Rust include signed and unsigned integers of different sizes, such as `i8` (signed 8-bit integer) and `u64` (unsigned 64-bit integer). Rust also has two floating-point types, `f32` and `f64`, 32 bit and 64 bit (better known as Float and Double), as well as a `bool` type for boolean values and a `char` type for Unicode characters. The possible values for integers include both of unsigned sizes (`u8`, `u16`, `u32`, `u64`) and signed integer sizes (`i8`, `i16`, `i32`, `i64`) with the added options of 128 bit integer sizes for unsigned and signed integers through Rusts `std` library

```Rust
let rust_float: f32 = 3.14; // Declaring And Initializing A Float (f32 32-bit) Variable
let rust_double: f64 = 3.1415; // Declaring And Initializing A Double (f64 64-bit) Variable
let example_flag: bool = false; // Declaring A boolean Value In Rust
let rust_char: char = 'A'; // Declaring And Initializing A char Variable
std::i128;
std::u128;
```

Rust also has built in variables for String values, they are `String` and `&str`. 

### Strings In Rust

In Rust, `String` and `&str` are both used to represent text data, but they have different characteristics and use cases.

#### String Type

`String` is a growable, heap-allocated data structure that represents a `UTF-8` encoded string of text. It is mutable and can be modified by appending or removing characters (Think StringBuilder in Java). You can create a new `String` by calling the `String::new()` method, or by converting from a string literal using the `to_string()` method:

```Rust
let mut empty_string = String::new() // Create Empty Appendable String Similar To StringBuilder
let mut string_from_literal = "My String Literal".to_string() // Turn String Literal Into StringBuilder Like Type

/*
To append to a String DataType one could use either statement
*/

s1.push_str("Hello, ");
s1 += "world!";
```

#### &str or String Slice

`&str` (pronounced "string slice") is an immutable view into a sequence of `UTF-8` bytes that represents a string of text. It does not own the data it points to, and it cannot be modified directly. You can create a `&str` by taking a slice of a `String` using the `&` operator, or by using a string literal:

```Rust
let s1 = String::from("Hello, world!");
let s2: &str = &s1; // creates a &str that points to the same data as s1
let s3: &str = "Hello, world!"; // creates a &str from a string literal
```

In general, it is best to use `String` when you need to own and modify a string of text, and use `&str` when you only need to view and manipulate a string of text without owning it. This distinction is important because it affects memory usage and performance, especially in cases where large strings of text are involved.

### Rusts Data Structures

Rust also includes several built-in data structures, such as vectors and hash maps. Vectors are similar to arrays in other programming languages, but with the added ability to dynamically grow or shrink in size as needed (So think ArrayLists in Java when thinking of Vectors), Rusts array data type is the exact same as standard arrays in Java. HashMaps are a key-value data structure that allows you to store and look up values based on a unique key.

#### Structs in Rust

In Rust, you can also define your own custom data types using structs, which are similar to C`s structs, enums, and traits. A struct is a way to group together multiple values of different types into a single type. For example, you could define a struct to represent a point in two-dimensional space:

```Rust
struct Point {
    x: f64,
    y: f64,
}
```

Here, `Point` is the name of the struct, and `x` and `y` are the names of the fields. You can then create new instances of the `Point` struct like so:

```Rust
let p = Point { x: 0.0, y: 0.0 };
```

#### Enums in Rust

Enums in Rust are similar to algebraic data types in functional programming languages. They allow you to define a type that can take on one of several possible values. For example, you could define an enum to represent the different states of a game:

```Rust
enum GameState {
    Initializing,
    Running,
    Paused,
    GameOver,
}
```

Here, GameState is the name of the enum, and the different states are defined as variants. You can then create new instances of the GameState enum like so:

```Rust
let state = GameState::Initializing;
```

#### Traits In Rust

Finally, Rust's trait system allows you to define behavior that can be shared across different types. Traits are similar to interfaces in other programming languages, but with more flexibility. We will see later how we can use these traits to mimic OOP in Rust

For example, you could define a Drawable trait that specifies how to draw an object on the screen:

```Rust
trait Drawable {
    fn draw(&self);
}

/*
Here, `Drawable` is the name of the trait, and `draw` is a method that takes a reference to `self` and returns nothing. You can then implement the `Drawable` trait for different types, such as the `Point` struct from earlier:
*/

impl Drawable for Point {
    fn draw(&self) {
        // Code to draw the point on the screen
    }
}
```

Here, `impl` is short for "implementation", and it defines how the `Drawable` trait should be implemented for the `Point` struct. Now, you can call the `draw` method on any instance of `Point` the same way you call methods on Objects in Java:

```Rust
let p = Point { x: 0.0, y: 0.0 };
p.draw();
```

##### Generic Traits

In Rust, traits can also have associated types, which allow you to define a type that is associated with the trait. For example, you could define a `Container` trait that represents a container of some kind, but where the type of the elements is not specified:

```Rust
trait Container {
    type Item;
    fn add(&mut self, item: Self::Item);
    fn remove(&mut self, item: Self::Item);
}
```
Here, `Container` is the name of the trait, and `Item` is an associated type that represents the type of the elements in the container. The `add` and `remove` methods take a mutable reference to `self` and a value of type `Self::Item`. Now, you can implement the `Container` trait for different types of containers, such as a `Vec`:

```Rust
impl<T> Container for Vec<T> {
    type Item = T;
    fn add(&mut self, item: T) {
        self.push(item);
    }
    fn remove(&mut self, item: T) {
        if let Some(pos) = self.iter().position(|x| *x == item) {
            self.remove(pos);
        }
    }
}
```

Here, the `impl<T>` syntax specifies that the implementation is generic over the type `T`. The `type Item = T`; line associates the `Item` type with the generic type `T`. Now, you can create a new `Vec` and add and remove elements from it using the `Container` trait methods:

```Rust
let mut v = Vec::new();
v.add(42);
v.add(1337);
v.remove(42);
```

### This Next Section Will Introduce Option<T> and Box<T>. We Will Then Compare These To There Counterparts in Java/C And Over The Course Of These Introductions, We Will Build A Basic LinkedList, Upon Understanding How These Components Are Used For The Creation Of LinkedLists In Rust. One Can Begin To Understand Implementing An OOP Like Style In Rust. First We Will Cover The Types And Their Basic Purposes. Then We Shall Show How They Interact During LinkedList Creation.

#### Option<T>

In languages like Java or C. A user may wish to designate a value as `null` for whatever purposes (Say the presence of a `null` or `nil` value in a variable is used to evaluate a conditional, say the final node of a LinkedList). Most users who start on Rust (whether new programmers or advanced programmers) sometimes struggle with the concept of being able to mark a value as `null`. For instance, without specifying `Option<T>` as I will show soon, a programmer is unable to assign any value to a string to mark the absence of a value other than assigning "" (Characterless String) to the variable. This is because most programmers do not go far enough to learn about what `Option<T>` is and provides. Declaring a variable as `Option<T>` gives that variable type the ability to be `null` or in the case of Rust, `None`. In Rust, any type can be declared as an Optional variable and can be done like so

```Rust
/*
Declaring an Optional String that can have a value or be None
*/
let mut optional_string: Option<String> = None;
```

The statement above declares an `Optional String` with a `null` (Again, for Rust its `None`) that is mutable which allows it to be assigned a possible value later. `Option<T>` is a powerful tool that helps Rust programmers to write safer and more robust code by providing a clear way to handle optional values. It is a widely used construct in Rust and is often used in combination with match statements to handle different possible outcomes.

#### Box<T>

In Java, when a primitive type is used in a context that requires an object, it is automatically "boxed" into an object on the heap. This process is called "autoboxing," and it allows the programmer to use primitive types where an object is expected, such as in generic collections or method calls that take Object parameters. For example, an int can be boxed into an Integer object automatically.

In most object-oriented programming languages, objects are typically stored on the heap. The heap is a region of memory that is reserved for dynamic memory allocation, meaning that objects can be created and destroyed at runtime as needed. When an object is created, memory is allocated on the heap to hold its data and any additional metadata, such as a vtable in languages like C++. The object's reference or pointer is typically stored on the stack or in a register.

In Rust, Box<T> is a type that allows the programmer to allocate values on the heap rather than on the stack. The Box<T> type is used to hold values of any type T that have a known size at compile time. When a value is boxed, it is stored on the heap, and a pointer to the value is returned. This allows the value to be moved around in memory without changing its location, which can be important for performance reasons. Box<T> is often used to store large data structures, such as trees or linked lists, that would be too large to fit on the stack.

In summary, while Javas implicit boxing automatically creates objects on the heap to represent primitive types, in Rust, the Box<T> type provides a way to explicitly allocate objects on the heap. Both approaches allow for dynamic memory allocation and can be useful in different contexts. I bring in Java for two reasons. 1 which is described in the second paragraph, and that is all Objects are stored on the heap for the majority of OOP style languages, and 2, Javas autoboxing feature which implicitly casts Primitive Types into Objects. These two features of Java and OOP as a whole will be brought in again later when we make our own `OOP` style code in Rust. For now, however, lets just show Rusts `Box<T>` in action.

##### When To Use Box<T> Normally?

Assume you are attempting to build a Recursive Data Structure in Rust, we will use a BinaryTree for this example. A BinaryTree consists of Nodes which themselves are trees (left, right nodes normally). As stated, Box<T> is used to explicitly create variables in the heap, without this, variables are always allocated on the stack. So without using Box<T>, attempting to create this recursive BinaryTree without allocating on the heap, Rust would cause two problems:

```Rust
struct BinaryTree {
    value: i32,
    left: Option<BinaryTree>,
    right: Option<BinaryTree>,
}
```

###### 1. Assuming it allowed the struct to be created, all nodes would be made in the stack which for most BinaryTrees would inevitably lead to a `Stack Overflow Error`

###### 2. Since creating the node struct involves variables of its Type to be stored in the struct, on the stack this would cause the struct to be infinitely sized due to the infinite recursion that is caused by itself being directly stored as values within itself.

###### Both of these are errors and should be avoided at all cost.

###### A proper example of using Box<T> and creating a `BinaryTree` struct is shown below

```Rust
struct BinaryTree {
    value: i32,
    left: Option<Box<BinaryTree>>,
    right: Option<Box<BinaryTree>>,
}
```

By wrapping the child nodes in Box<T>, we can allocate them on the heap and store a pointer to them in the parent struct. This way, the size of the BinaryTree struct is fixed and does not depend on the depth of the tree. Note that simply replacing `BinaryTree` with `Node` as well as left/right with Prev/Next, we could use this to construct our `Nodes` for a LinkedList

Using Box<T> in this way allows us to define recursive data structures in Rust without running into issues with infinite types. It also allows us to store large data structures on the heap, which can be more efficient than storing them on the stack.

###### Notice how in both examples the left and right child nodes (which are the node structs themselves) are declared as Optional first, this ensures that the nodes can hold `null` values as Trees in actual programming can exist without any children.

#### The LinkedList

Now that introductions out of the way lets see a LinkedList in Java with only push and pop methods implemented and then compare that to a fully functioning LinkedList in Rust that as well, only has push and pop methods implemented. 

#####  Notes For The LinkedLists That Are Below

##### 1. Both of them will be Generic.
##### 2. Neither Will Check For Null Data So Assume Any Time A New Node Is Made The Data Passed Through Contains Any Data That Is Not Null

###### Java LinkedList

```Java
public class LinkedList<T> {
    
    private Node<T> head;

    public LinkedList() {
        head = null;
    }
    
    public void push(T data) {
        Node<T> newNode = new Node<T>(data);
        newNode.next = head;
        head = newNode;
    }
    
    public T pop() {
        if (head == null) {
            return null;
        }
        T data = head.data;
        head = head.next;
        return data;
    }
    
    private static class Node<T> {
        private T data;
        private Node<T> next;

        public Node(T data) {
            this.data = data;
            next = null;
        }
    }
}
```
In This LinkedList, we declare a Generic parent-class called LinkedList that has a Node subclass. The subclass is used to create the Node Objects that are stored on the LinkedList. Push creates a new subclass instance (object) which stores the node pointed at the beginning of the list as its next node (This pushes the new node to the front like a stack). Notice how nothing is needed to specify `null` values

###### Rust LinkedList

```Rust
struct LinkedList<T> {
    head: Option<Box<Node<T>>>,
}

struct Node<T> {
    data: T,
    next: Option<Box<Node<T>>>,
}

impl<T> LinkedList<T> {
    fn new() -> Self {
        LinkedList { head: None }
    }
    
    fn push(&mut self, data: T) {
        let new_node = Node { data: data, next: self.head.take() };
        self.head = Some(Box::new(new_node));
    }
    
    fn pop(&mut self) -> Option<T> {
        self.head.take().map(|node| {
            self.head = node.next;
            node.data
        })
    }
}
```

In this example, the LinkedList struct is also parameterized with a type parameter `T`, which allows it to be used with any type of data. The struct includes an inner Node struct to represent the individual nodes of the list. The `new` method creates a new empty list, while the `push` method adds a new node to the beginning of the list. The `pop` method removes and returns the data from the head of the list, using Rust's `Option` type to handle the case where the list is empty. Classes are not a thing in Rust so we must use structs to store the node and LinkedList Data. You may be wondering why we didn`t use a trait since I said that was important for our Mock OOP in Rust, in this case it was not used as I just wanted to show a basic LinkedList. You may also be wondering what the `self.head.take()` statement is doing. Simply put, it is taking the `head` node and all nodes linked to it, moving it into the `next` value of the new node. In essence, the `push` method returns a `new_node` that is basically a new list each time. The `self` parameter is no different than `self` in Python.

Now that we have gotten all of our introductions out of the way. Lets dive into Object Oriented Programming and what it is. Then we can see how to mimic OOP in Rust

## What exactly IS Object Oriented Programming

Object-oriented programming (OOP) is a programming paradigm that is based on the concept of "objects", which can contain data and code to manipulate that data. It is a style of programming that allows for the creation of reusable and modular code, making it easier to develop and maintain software.

At the heart of OOP is the idea that a software system can be broken down into individual components, called objects, which can interact with each other to perform specific tasks. Each object has its own set of data, known as its attributes or properties, and a set of methods, which are functions that operate on that data. The methods define the behavior of the object, determining how it responds to various actions and interactions.

The main features of OOP are:

1. Encapsulation: This refers to the practice of hiding the internal workings of an object from the outside world, and only exposing a public interface that can be used to interact with the object. This makes it easier to change the implementation of an object without affecting other parts of the program.
2. Inheritance: This allows for the creation of new objects that are based on existing ones, inheriting their properties and methods. This can help to reduce code duplication and make it easier to manage complex software systems.
3. Polymorphism: This refers to the ability of objects to take on multiple forms, depending on the context in which they are used. This allows for more flexibility and reuse of code.

Some of the benefits of using OOP include:

- Modular code: OOP allows for the creation of reusable and modular code, which can be easily maintained and updated.
- Code reusability: By using inheritance and polymorphism, code can be reused across different parts of a program, reducing the amount of code that needs to be written and making it easier to maintain.
- Flexibility: OOP allows for more flexible and adaptable software, as objects can be easily modified and extended as needed.
- Better organization: By breaking a program down into objects, the code can be organized in a more logical and structured way, making it easier to understand and maintain.

These features are the backbone to Object Oriented Programming and to be a full OOP style language, it must be have Encapsulation, Inheritance, and Polymorphism. Rust is able to meet some of these features, however one key feature is not met and we will go over here.

### What Is Rust Then? What Does Rust Lack For OOP?

Rust is a multi-paradigm programming language that emphasizes performance, memory safety, and thread safety. While Rust is not strictly an object-oriented programming language, it provides some features that can be used to achieve object-oriented programming (OOP) concepts.

One of the main features that Rust lacks in OOP is inheritance. Rust does not have a native way to implement class inheritance, which is a core concept in OOP. However, Rust provides a way to achieve code reuse through traits. Traits are similar to interfaces in other programming languages, defining a set of methods that a type must implement to conform to the trait.

Another aspect that Rust lacks in OOP is dynamic dispatch. In OOP, dynamic dispatch allows a program to call a method on an object without knowing its exact type at compile time. Rust, on the other hand, is a statically-typed language, which means that the type of a variable must be known at compile time. However, Rust provides a way to achieve dynamic dispatch through the use of trait objects. A trait object is a type-erased pointer to a type that implements a certain trait, allowing for dynamic dispatch of trait methods.

To achieve mock OOP in Rust, one can use a combination of traits, structs, and methods. Traits can be used to define a set of methods that an object must implement, while structs can be used to encapsulate data and implement the methods defined in the trait. Methods can then be defined on the struct to operate on the data encapsulated within it. By using traits to define the interface and structs to encapsulate data and implement methods, Rust can achieve some of the benefits of OOP, such as code reuse, encapsulation, and polymorphism, while still maintaining its emphasis on performance and memory safety.

The main feature missing however, is that of inheritance. Inheritance is a critical feature of OOP and is the main reason why Rust is not able to achieve a perfect OOP style. But first in order to understand why Inheritance is so crucial to OOP. One must first understand what Inheritance is.

### What Exactly Is Inheritance Then?

Inheritance is a core concept in object-oriented programming (OOP) that allows for the creation of new classes that inherit the properties and methods of an existing class. Inheritance enables code reuse and helps to organize code into a hierarchical structure.

Inheritance works by creating a parent-child relationship between classes. The parent class, also called the superclass or base class, is the class that contains the common properties and methods that are shared by its child classes. The child class, also called the subclass or derived class, is the class that inherits those properties and methods from the parent class and can also add its own properties and methods.

To create a child class that inherits from a parent class, the child class is defined using the parent class as a template. This is done using the "extends" keyword in many OOP languages. The child class automatically inherits all of the properties and methods of the parent class, and can also override or add to them as needed. In addition, the child class can define its own properties and methods that are specific to it.

Inheritance can be used to create a hierarchy of classes, with each class inheriting from the class above it in the hierarchy. This can help to organize code and make it easier to maintain, as common properties and methods are shared by multiple classes.

One of the main benefits of inheritance is code reuse. By defining common properties and methods in a parent class, those properties and methods can be reused by multiple child classes, reducing code duplication and making it easier to maintain the code.

Inheritance can also provide a way to achieve polymorphism, which is the ability of objects to take on multiple forms. By defining a common interface in the parent class, child classes can be treated as instances of the parent class, allowing for more flexible and generic code.

However, it's important to use inheritance judiciously, as overuse of inheritance can lead to overly complex and tightly-coupled code. Inheritance can also introduce tight coupling between classes, making it difficult to modify or extend the code in the future.

In Java, inheritance is achieved through the use of the "extends" keyword. To inherit from a class, the child class is declared using the following syntax:

```Java
class ChildClassName extends ParentClassName {
    // class body
}
```

Here, the "ChildClassName" is the name of the new class, and "ParentClassName" is the name of the existing class that the new class will inherit from.

When a class inherits from another class, it automatically gains access to all of the public and protected properties and methods of the parent class. For example, consider the following parent class:

```Java
public class Animal {
    protected String name;
    
    public Animal(String name) {
        this.name = name;
    }
    
    public void speak() {
        System.out.println("I am an animal.");
    }
}
```

This class defines a single property, "name", and a single method, "speak()". Now, we can create a child class that inherits from this parent class:

```Java
public class Dog extends Animal {
    public Dog(String name) {
        super(name);
    }
    
    public void speak() {
        System.out.println("Woof!");
    }
}
```

Here, the "Dog" class extends the "Animal" class using the "extends" keyword. The constructor of the "Dog" class calls the constructor of the parent class using the "super()" method, passing in the name of the dog. The "Dog" class also overrides the "speak()" method of the parent class to provide its own implementation.

Now, we can create instances of both classes and see how they behave:

```Java
Animal animal = new Animal("Generic Animal");
Animal dog = new Dog("Fido"); // Can Also Be Created With `Dog dog = new Dog("Fido")`

System.out.println(animal.name); // "Generic Animal"
animal.speak(); // "I am an animal."

System.out.println(dog.name); // "Fido"
dog.speak(); // "Woof!"
```

Here, we create an instance of the "Animal" class and the "Dog" class. We can see that the "Animal" instance has access to the "name" property and "speak()" method of the parent class, while the "Dog" instance has access to both the "name" property and "speak()" method of the parent class, as well as its own "speak()" method implementation.

### So Rust doesn`t have Inheritance at all?

Rust does not have textbook Inheritance. This is the main reason Rust is unable to achieve an OOP style. However there are mechanisms in Rust that allow for "mimicking" OOP and Inheritance. 

#### Composition

One such mechanism is composition, where one struct contains another struct as a field. This allows the containing struct to delegate functionality to the contained struct. For example, suppose we have a struct `Person` that has a name and an age:

```Rust
struct Person {
    name: String,
    age: u8,
}
```

Now, suppose we want to create a struct `Employee` that has all of the fields of `Person`, as well as an additional `salary` field. We can achieve this through composition by including a `Person` field within the `Employee` struct:

```Rust
struct Employee {
    person: Person,
    salary: f32,
}
```

Now, the `Employee` struct contains all of the fields of the `Person` struct, and we can access them using dot notation, like this:

```Rust
let john = Person { name: "John".to_string(), age: 30 };
let john_doe = Employee { person: john, salary: 50000.0 };

println!("Name: {}", john_doe.person.name);
println!("Age: {}", john_doe.person.age);
println!("Salary: {}", john_doe.salary);
```

Here, we create a `Person` struct called `john` and an `Employee` struct called `john_doe`, which contains a `Person` struct as a field. We can access the fields of the `Person` struct using dot notation on the `person` field of the `Employee` struct. If we wanted to, we could also add an Implementation to the Employee Struct to further mimic the inheritance, like so:

```Rust
struct Person {
    name: String,
    age: u32,
}

struct Employee {
    person: Person,
    job_title: String,
    salary: f64,
}

impl Employee {
    fn new(name: String, age: u32, job_title: String, salary: f64) -> Employee {
        Employee {
            person: Person {
                name: name,
                age: age,
            },
            job_title: job_title,
            salary: salary,
        }
    }

    fn describe(&self) {
        println!("{} is a {} earning ${} per year.", self.person.name, self.job_title, self.salary);
    }
}
```
Here, `Employee` includes a `Person` field, allowing it to access the `name` and `age` fields of a `Person`. Additionally, `Employee` has its own fields `job_title` and `salary`, and its own methods like `new` and `describe`.

We can create an instance of `Employee` using the `new` method, and call its `describe` method to print out a description of the employee:

```Rust
let employee = Employee::new(String::from("Alice"), 30, String::from("Software Developer"), 100000.0);
employee.describe();
```
Which will output the following:

```
Alice is a Software Developer earning $100000 per year.
```

Thus, via composition, we can somewhat achieve a mimicked inheritance in Rust. But there are other ways than composition. We can also use traits to mimic inheritance similar to Javas Interfaces

#### Traits

Another mechanism in Rust for achieving functionality similar to inheritance is the use of traits. As mentioned before, a trait is a collection of methods that can be implemented by a type. We can create a trait that defines a set of methods, and then implement that trait for different types. This allows us to define a set of behavior that can be shared across multiple types. For example, suppose we have a trait `Animal` that defines a method `speak()`:

```Rust
trait Animal {
    fn speak(&self);
}
```

Now, we can implement this trait for different types of animals. For example, we can implement the `Animal` trait for a `Dog` struct:

```Rust
struct Dog {
    name: String,
}

impl Animal for Dog {
    fn speak(&self) {
        println!("Woof!");
    }
}
```

Here, we define a `Dog` struct that has a `name` field, and we implement the `Animal` trait for the `Dog` struct. The `speak()` method of the `Animal` trait is defined to print "Woof!" to the console.

Now, we can create instances of the `Dog` struct and call the `speak()` method on them:

```Rust
let fido = Dog { name: "Fido".to_string() };

fido.speak();
```

Now, we can create instances of the `Dog` struct and call the `speak()` method on them:

Here, we create a `Dog` struct called `fido` and call the `speak()` method on it. The `speak()` method is defined by the `Animal` trait, which the `Dog` struct implements.

Using these two mechanisms, we can somehow mimic inheritance which can be used later in order to mimic an Object Oriented Implementation.

### Lastly, Modules In Rust

Modules in Rust are a way of organizing code into namespaces and controlling visibility of functions, structs, enums, and other items. Modules allow us to group related code together and make it more manageable, as well as prevent naming conflicts between items with the same name. In this way, modules provide a means of organizing Rust code that is similar to the class system in Java.

In Rust, a module can be defined using the `mod` keyword, followed by the name of the module, like this:

```Rust
mod my_module {
    // code goes here
}
```

The code within a module is encapsulated and can only be accessed by code within the same module or by code that has been specifically allowed to access it using the `pub` keyword. For example, we can define a struct within a module and make it publicly visible using the `pub` keyword:

```Rust
mod my_module {
    pub struct MyStruct {
        // fields go here
    }
}
```

Now, the `MyStruct` struct is visible to code outside of the `my_module` module, and can be used in other modules or in the main code.

Modules in Rust can also be used to achieve functionality similar to inheritance through the use of traits and composition. For example, we can define a trait called `Animal` that defines a method called `speak()`:

```Rust
pub trait Animal {
    fn speak(&self);
}
```

Now, we can define a module called `dog` that contains a struct called `Dog` that implements the `Animal` trait:

```Rust
mod dog {
    use super::Animal;

    pub struct Dog {
        name: String,
    }

    impl Animal for Dog {
        fn speak(&self) {
            println!("Woof!");
        }
    }
}
```

Here, we use the `super` keyword to refer to the parent module (which is the module that contains the `dog` module). We also use the `use` keyword to bring the `Animal` trait into scope so that we can implement it for the `Dog` struct. Finally, we define the `Dog` struct and implement the `Animal` trait for it by defining the `speak()` method to print "Woof!" to the console.

Now, we can use the `Dog` struct and its `speak()` method in other modules or in the main code. For example:

```Rust
use dog::Dog;
use dog::Animal;

fn main() {
    let fido = Dog { name: "Fido".to_string() };

    fido.speak();
}
```

Here, we use the `use` keyword to bring the `Dog` struct and `Animal` trait into scope, and then create an instance of the `Dog` struct called `fido`. We then call the `speak()` method on `fido`, which is defined by the `Animal` trait.

## Combining All Together To Mock Object Oriented Program

Now we are going to be making two programs. Our first program will be a simple program of 3 types of Cars that inherit from a Vehicle/Car module. One type will be a truck that we will have child modules as well. The 4th will be a child module that can store all types of Vehicles.

Our first module is Automotive and this will act as our parent class. I have defined it below

```Rust
// Parent module containing a trait and a struct
pub mod Automotive {
    pub trait Automotive {
        fn start(&self);
        fn stop(&self);
    }

    #[derive(Clone)]
    pub struct Vehicle {
        pub name: String,
        pub year: i32,
    }

    impl Automotive for Vehicle {
        fn start(&self) {
            println!("{} started", self.name);
        }

        fn stop(&self) {
            println!("{} stopped", self.name);
        }
    }
}
```
Here we defined a module called Automotive. In it we declared a trait Automotive that acts as our interface with a struct that will act as the parent class that is the first generation of implementations. Next we will define our 3 main child modules that directly inherit from our Automotive Module.

First we will create a SportsCar submodule

```Rust
// First child module SportsCar with a struct that contains an instance of the parent struct
pub mod SportsCar {
    use crate::OOP_Trucks::Automotive::{Automotive, Vehicle};
    pub struct SportsCar {
        pub car: Box<Vehicle>,
        pub top_speed: Box<f64>,
    }

    impl Automotive for SportsCar {
        fn start(&self) {
            println!("{} Sports Car Started", self.car.name);
        }

        fn stop(&self) {
            println!("{} Sports Car Started", self.car.name);
        }
    }
}
```

Next is our SUV submodule

```Rust
// Second child module (SUV) with a struct that composes a parent struct and a new field
pub mod SUV {
    use crate::OOP_Trucks::Automotive::{Automotive, Vehicle};
    pub struct SUV {
        pub car: Box<Vehicle>,
        pub cargo_capacity: Option<Box<i32>>,
    }

    impl Automotive for SUV {
        fn start(&self) {
            println!("{} SUV Started", self.car.name);
        }

        fn stop(&self) {
            println!("{} SUV Started", self.car.name);
        }
    }
}
```

Our last separate vehicle type is Truck, which will have its own child modules

```Rust
// Third child module (truck) that has its own child modules pickup truck, cement truck, tractor trailer, and fire truck
pub mod Truck {
    use crate::OOP_Trucks::Automotive::{Automotive, Vehicle};

    #[derive(Clone)]
    pub struct Truck {
        pub car: Box<Vehicle>,
        pub cargo_capacity: Option<Box<i32>>,
    }

    impl Automotive for Truck {
        fn start(&self) {
            println!("{} truck started", self.car.name);
        }

        fn stop(&self) {
            println!("{} truck stopped", self.car.name);
        }
    }
}
```
##### Truck Child Modules

Our first child module of truck will be PickupTruck

```Rust
// First Child Module (PickupTruck) of Truck
pub mod PickupTruck {
use crate::OOP_Trucks::{Automotive::*, Truck::Truck};
    pub struct PickupTruck {
        pub truck: Box<Truck>,
        pub bed_size: Option<Box<f64>>,
        pub is_4wd: Box<bool>,
    }

        impl Automotive for PickupTruck {
        fn start(&self) {
            println!("{} pickup truck started", self.truck.car.name);
        }

            fn stop(&self) {
            println!("{} pickup truck stopped", self.truck.car.name);
        }
    }
}
```

Next we will implement a child module for TowTruck

```Rust
// Second Child Module (TowTruck) of Truck
pub mod TowTruck {
    use crate::OOP_Trucks::{Automotive::*, Truck::Truck};
    pub struct TowTruck {
        pub truck: Box<Truck>,
        pub bed_size: Option<Box<f64>>,
        pub is_flatbed: Box<bool>,
    }

    impl Automotive for TowTruck {
        fn start(&self) {
            println!("{} TowTruck started", self.truck.car.name);
        }

        fn stop(&self) {
            println!("{} TowTruck stopped", self.truck.car.name);
        }
    }
}
```

Lastly we will implement one final child class

```Rust
// Third Child Module (TractorTrailer) of Truck
pub mod TractorTrailer {
    use crate::OOP_Trucks::{Automotive::*, Truck::Truck};
    pub struct TractorTrailer {
        pub truck: Box<Truck>,
        pub trailer_length: Option<Box<f64>>,
        pub is_double: Box<bool>,
    }

    impl Automotive for TractorTrailer {
        fn start(&self) {
            println!("{} TractorTrailer started", self.truck.car.name);
        }

        fn stop(&self) {
            println!("{} TractorTrailer stopped", self.truck.car.name);
        }
    }
}
```

##### Next we implement a child module of the original automotive module that can take any vehicle

```Rust
// Fourth child module (Fleet) with a struct that composes a parent struct and a vector of child structs
pub mod Fleet {
    use crate::OOP_Trucks::Automotive::{Automotive, Vehicle};

    pub struct Fleet {
        pub cars: &'static Vec<Box<dyn Automotive>>,
        pub company_name: Option<Box<String>>,
    }

    impl Automotive for Fleet {
        fn start(&self) {
            println!("{:?} fleet is starting up", self.company_name);
            for i in 0..self.cars.len() - 1{
                if i == self.cars.len() {
                    break;
                }
                self.cars.get(i).unwrap().start();
            }
        }

        fn stop(&self) {
            println!("{:?} fleet is shutting down", self.company_name);
            for i in 0..self.cars.len() - 1 {
                if i == self.cars.len() {
                    break;
                }
                self.cars.get(i).unwrap().stop();
            }
        }
    }
}
```

This is how we can implement a full OOP style program with "subclasses" that "inherit" from a parent "class"

We can do the same for anything once we understand this concept. For instance below is an implementation for a LinkedList trait (Similar to a LinkedList Interface in Java).

Note: This is implemented using vectors. A more complex implementation (As above using full nodes would not require much changes)

##### LinkedList OOP

Below is our LinkedList Trait in our LinkedList Parent Module

```Rust
pub mod linked_list {
    pub trait LinkedList<T> {
        fn push(&mut self, value: T);
        fn pop(&mut self) -> Option<T>;
    }
}
```

For our first implementation we will create a stack. See below

```Rust
pub mod stack {
    use super::linked_list::LinkedList;

    pub struct Stack<T> {
        list: Vec<T>,
    }

    impl<T> Stack<T> {
        pub fn new() -> Self {
            Self { list: vec![] }
        }
    }

    impl<T> LinkedList<T> for Stack<T> {
        fn push(&mut self, value: T) {
            self.list.push(value);
        }

        fn pop(&mut self) -> Option<T> {
            self.list.pop()
        }
    }
}
```

The second implementation will be a queue

```Rust
pub mod queue {
    use super::linked_list::LinkedList;

    pub struct Queue<T> {
        list: Vec<T>,
    }

    impl<T> Queue<T> {
        pub fn new() -> Self {
            Self { list: vec![] }
        }
    }

    impl<T> LinkedList<T> for Queue<T> {
        fn push(&mut self, value: T) {
            self.list.push(value);
        }

        fn pop(&mut self) -> Option<T> {
            if self.list.is_empty() {
                None
            } else {
                Some(self.list.remove(0))
            }
        }
    }
}
```

Lastly we will implement a Doubly Linked List Which Employs Nodes Rather Than a Vec<T>

```Rust
pub mod doubly_linked_list {
    use super::linked_list::LinkedList;

    pub struct Node<T> {
        value: T,
        next: Option<Box<Node<T>>>,
        prev: Option<*mut Node<T>>,
    }

    pub struct DoublyLinkedList<T> {
        head: Option<Box<Node<T>>>,
        tail: Option<*mut Node<T>>,
    }

    impl<T> DoublyLinkedList<T> {
        pub fn new() -> Self {
            Self {
                head: None,
                tail: None,
            }
        }
    }

    impl<T> LinkedList<T> for DoublyLinkedList<T> {
        fn push(&mut self, value: T) {
            let mut new_node = Box::new(Node {
                value,
                next: None,
                prev: None,
            });

            if let Some(mut tail) = self.tail {
                unsafe {
                    (*new_node).prev = Some(tail);
                    (*tail).next = Some(new_node);
                }
            } else {
                self.head = Some(new_node);
            }

            self.tail = Some(Box::into_raw(new_node));
        }

        fn pop(&mut self) -> Option<T> {
            if let Some(tail) = self.tail.take() {
                let tail = unsafe { Box::from_raw(tail) };

                if let Some(prev) = tail.prev {
                    unsafe {
                        (*prev).next = None;
                        self.tail = Some(prev);
                    }
                } else {
                    self.head = None;
                }

                Some(tail.value)
            } else {
                None
            }
        }
    }
}
```

These are how we can do OOP in Rust by Mimicking Inheritance to implement an OOP like implementation. Using 'Barebones Programming' we practically implement it from scratch. I hope you have found a new enjoyment for Rust and a better understanding of programming at an internal level.
See source code to run it yourself :)
