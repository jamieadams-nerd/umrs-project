# Robot Generator: Convert ASCII Art to Static Struct

This tool converts an ASCII art file to a static structure for use by the umrs-core library. It has a umrs_core::robots::builtins module to store static structure. 

Why? So, we can display a cool robot when we want from any code that is using the umrs-core library. Cool? Hell, yeah. So, this tool was written to convert text files to structures to be compoiled into the core library. 


## Converting an art file

For example, given a text file, we use our tool to convert it to we Rust code structure. Below is the content of the **tiny_robot.txt** file:

```
      \_/
     (* *)
    __)#(__
   ( )...( )(_)
   || |_| ||//
>==() | | ()/
    _(___)_
   [-]   [-]MJP
```

The **umrs-robotgen** command accepts two arguments: ```<NAME> <TEXTFILE>```.
The NAME is the name of the structure so keep it simple with no spaces and is
Rust compliant. 

Inside the umrs-robotgen crate (or use the binary standalong). 

```cargo run -- tiny_robot tiny_robot.txt```

It creates a defeinition which every row has the same number of columns while
also trimming whitespace from the right and left sides of the widest line. 

It will product the following:

```
pub static TINY_ROBOT: AsciiArtStatic = AsciiArtStatic {
    name: "tiny_robot",
    width: 15,
    height: 8,
    lines: &[
        "      \\_/      ",
        "     (* *)     ",
        "    __)#(__    ",
        "   ( )...( )(_)",
        "   || |_| ||// ",
        ">==() | | ()/  ",
        "    _(___)_    ",
        "   [-]   [-]MJP",
    ],
};
```

## Add the results the Rust module

Add the above code into the **umrs_core::robots::builtins* module.
Specifically, the to the umrs-core/src/robots/builtins.rs file.

Since the ```cargo run``` prints the results to standard out, you might want to
redirect it into a temporary file. Then append it to the builtins.rs.
Otherwise, just copy and paste. 

Once added, you should build the umrs-core to make sure you didn't break
anything. You can do that from the workspace, umrs-core top-lelvel, or in the
umrs-robotgen by executing the following:

```carbo build```


