/// map VM segments to their corresponding assembler symbols
pub fn segment_mapping(segment: String) -> String {
    match segment.as_str() {
        "local" => "LCL",
        "argument" => "ARG",
        "this" => "THIS",
        "that" => "THAT",
        _ => "error: invalid segment",
    }
    .to_string()
}

/// implementations of the VM's arithmetic commands
/// command: the arithmetic command to be translated
/// cnt: counter
pub fn arithmetic_command(command: String, cnt: u32) -> String {
    match command.as_str() {
        // add the top two elements on the stack
        // impl desc:
        // select A = stack pointer
        // decrement stack pointer
        // select A = stack pointer
        // push the value of M to the D register
        // set A = A - 1
        // add the value of D to the value of M
        // thus effectively adding both numbers previously pushed
        // to the stack
        "add" => "@SP\n\
                  M=M-1\n\
                  A=M\n\
                  D=M\n\
                  A=A-1\n\
                  M=M+D\n"
            .to_string(),

        // subtract the top two elements on the stack
        // impl desc:
        // same as with add but subtract isntead of add
        "sub" => "@SP\n\
                  M=M-1\n\
                  A=M\n\
                  D=M\n\
                  A=A-1\n\
                  M=M-D\n"
            .to_string(),

        // negate the top element on the stack
        "neg" => "@SP\n\
                 A=M-1\n\
                 M=-M\n"
            .to_string(),

        // compare the top two elements on the stack
        // if they are equal, set the top element to -1
        // if they are not equal, set the top element to 0
        "eq" => format!(
            "@SP\n\
                    M=M-1\n\
                    A=M\n\
                    D=M\n\
                    A=A-1\n\
                    D=M-D\n\
                    @ISEQUAL{0}\n\
                    D;JEQ\n\
                    @NOTEQUAL{0}\n\
                    D;JNE\n\
                    (ISEQUAL{0})\n\
                    @SP\n\
                    A=M-1\n\
                    M=-1\n\
                    @EQEND{0}\n\
                    0;JMP\n\
                    (NOTEQUAL{0})\n\
                    @SP\n\
                    A=M-1\n\
                    M=0\n\
                    @EQEND{0}\n\
                    0;JMP\n\
                    (EQEND{0})\n",
            cnt
        ),

        // negate the top element on the stack
        "not" => "@SP\n\
                 A=M-1\n\
                 M=!M\n"
            .to_string(),

        // bitwise and the top two elements on the stack
        "and" => "@SP\n\
                M=M-1\n\
                A=M\n\
                D=M\n\
                A=A-1\n\
                M=M&D\n"
            .to_string(),

        // bitwise or the top two elements on the stack
        "or" => "@SP\n\
                M=M-1\n\
                A=M\n\
                D=M\n\
                A=A-1\n\
                M=M|D\n"
            .to_string(),

        // compare the top two elements on the stack
        // if the first element is greater than the second
        // set the top element to -1
        // otherwise set the top element to 0
        "gt" => format!(
            "@SP\n\
                    M=M-1\n\
                    A=M\n\
                    D=M\n\
                    A=A-1\n\
                    D=M-D\n\
                    @ISGREATER_GT{0}\n\
                    D;JGT\n\
                    @ISLESS_GT{0}\n\
                    D;JLE\n\
                    (ISGREATER_GT{0})\n\
                    @SP\n\
                    A=M-1\n\
                    M=-1\n\
                    @GTEND{0}\n\
                    0;JMP\n\
                    (ISLESS_GT{0})\n\
                    @SP\n\
                    A=M-1\n\
                    M=0\n\
                    @GTEND{0}\n\
                    0;JMP\n\
                    (GTEND{0})\n",
            cnt
        ),

        // compare the top two elements on the stack
        // if the first element is less than the second
        // set the top element to -1
        // otherwise set the top element to 0
        "lt" => format!(
            "@SP\n\
                    M=M-1\n\
                    A=M\n\
                    D=M\n\
                    A=A-1\n\
                    D=M-D\n\
                    @ISGREATER_LT{0}\n\
                    D;JGE\n\
                    @ISLESS_LT{0}\n\
                    D;JLT\n\
                    (ISGREATER_LT{0})\n\
                    @SP\n\
                    A=M-1\n\
                    M=0\n\
                    @LTEND{0}\n\
                    0;JMP\n\
                    (ISLESS_LT{0})\n\
                    @SP\n\
                    A=M-1\n\
                    M=-1\n\
                    @LTEND{0}\n\
                    0;JMP\n\
                    (LTEND{0})\n",
            cnt
        ),

        _ => format!("error: invalid arithmetic command"),
    }
}

/// implementations of the VM's push command
/// segment: the segment to be pushed to
/// address: the address to be pushed
/// filename: the name of the file
pub fn push_command(segment: String, address: String, filename: String) -> String {
    match segment.as_str() {
        // match the four segments with an identical implementation
        // just allocate using the assembler's A command
        // to LCL, ARG, THIS, or THAT
        "local" | "argument" | "this" | "that" => format!(
            "@{0}\n\
                D=A\n\
                @{1}\n\
                D=D+M\n\
                @addr\n\
                M=D\n\
                A=M\n\
                D=M\n\
                @SP\n\
                A=M\n\
                M=D\n\
                @SP\n\
                M=M+1\n",
            address,
            segment_mapping(segment.clone())
        ),

        // push a constant to the stack
        "constant" => format!(
            "@{0}\n\
                D=A\n\
                @SP\n\
                A=M\n\
                M=D\n\
                @SP\n\
                M=M+1\n",
            address
        ),

        // push a value to the temp segment
        "temp" => format!(
            "@{0}\n\
                D=A\n\
                @5\n\
                D=D+A\n\
                A=D\n\
                D=M\n\
                @SP\n\
                A=M\n\
                M=D\n\
                @SP\n\
                M=M+1\n",
            address
        ),

        // push a value to the static segment
        // using the following syntax:
        // @{filename}.{address}
        "static" => format!(
            "@{1}.{0}\n\
                D=M\n\
                @SP\n\
                A=M\n\
                M=D\n\
                @SP\n\
                M=M+1\n",
            address,
            filename.clone()
        ),

        // push a value to the specified pointer segment
        // only 2 values are allowed: 0 (THIS) and 1 (THAT)
        "pointer" => {
            let memory_idx = if address == "0" { "THIS" } else { "THAT" };

            format!(
                "@{0}\n\
                D=M\n\
                @SP\n\
                A=M\n\
                M=D\n\
                @SP\n\
                M=M+1\n",
                memory_idx
            )
        }
        _ => format!("todo push segment!"),
    }
}

/// implementations of the VM's pop command
pub fn pop_command(segment: String, address: String, filename: String) -> String {
    match segment.as_str() {
        // same as push but instead of pushing
        // we pop from the correct segment
        "local" | "argument" | "this" | "that" => format!(
            "@{0}\n\
                D=A\n\
                @{1}\n\
                D=D+M\n\
                @addr\n\
                M=D\n\
                @SP\n\
                M=M-1\n\
                A=M\n\
                D=M\n\
                @addr\n\
                A=M\n\
                M=D\n",
            address,
            segment_mapping(segment.clone())
        ),

        "temp" => format!(
            "@{0}\n\
                D=A\n\
                @5\n\
                D=D+A\n\
                @addr\n\
                M=D\n\
                @SP\n\
                M=M-1\n\
                A=M\n\
                D=M\n\
                @addr\n\
                A=M\n\
                M=D\n",
            address
        ),

        "static" => format!(
            "@SP\n\
                M=M-1\n\
                A=M\n\
                D=M\n\
                @{1}.{0}\n\
                M=D\n",
            address,
            filename.clone()
        ),

        "pointer" => {
            let memory_idx = if address == "0" { "THIS" } else { "THAT" };

            format!(
                "@SP\n\
                M=M-1\n\
                A=M\n\
                D=M\n\
                @{0}\n\
                M=D\n",
                memory_idx
            )
        }
        _ => format!("todo push segment!"),
    }
}

/// implementations of the VM's branching commands
/// arg1: the first argument (label, goto or if-goto)
/// arg2: the second argument (label)
pub fn branching_command(arg1: String, arg2: String) -> String {
    match arg1.as_str() {
        // (LABEL) command
        "label" => format!("({})\n", arg2),
        // goto command
        "goto" => format!("@{}\n0;JMP\n", arg2),
        // if-goto command
        // jump if value on top of stack is not zero
        "if-goto" => format!("@SP\nM=M-1\nA=M\nD=M\n@{}\nD;JNE\n", arg2),
        _ => format!("error: invalid branch"),
    }
}

/// implementations of the VM's function call command
/// fun: the function to be called
/// args: the number of arguments
/// cnt: counter
pub fn funcall(fun: String, args: String, cnt: u32) -> String {
    format!(
        "// push return address\n\
        @{0}_RETURN_{2}\n\
        D=A\n\
        @SP\n\
        A=M\n\
        M=D\n\
        @SP\n\
        M=M+1\n\
        // push LCL\n\
        @LCL\n\
        D=M\n\
        @SP\n\
        A=M\n\
        M=D\n\
        @SP\n\
        M=M+1\n\
        // push ARG\n\
        @ARG\n\
        D=M\n\
        @SP\n\
        A=M\n\
        M=D\n\
        @SP\n\
        M=M+1\n\
        // push THIS\n\
        @THIS\n\
        D=M\n\
        @SP\n\
        A=M\n\
        M=D\n\
        @SP\n\
        M=M+1\n\
        // push THAT\n\
        @THAT\n\
        D=M\n\
        @SP\n\
        A=M\n\
        M=D\n\
        @SP\n\
        M=M+1\n\
        // ARG = SP - 5 - nArgs\n\
        @SP\n\
        D=M\n\
        @5\n\
        D=D-A\n\
        @{1}\n\
        D=D-A\n\
        @ARG\n\
        M=D\n\
        // LCL = SP\n\
        @SP\n\
        D=M\n\
        @LCL\n\
        M=D\n\
        // goto function\n\
        @{0}\n\
        0;JMP\n\
        // label\n\
        ({0}_RETURN_{2})\n",
        fun, args, cnt
    )
}

/// implementations of the VM's function declaration command
pub fn fundecl(fun: String, args: String) -> String {
    let local_init: &str = "@SP\nA=M\nM=0\n@SP\nM=M+1\n";
    let mut block: String = String::new();
    for _i in 0..args.parse::<i32>().unwrap() {
        block.push_str(local_init);
    }
    format!("// function declaration\n({0})\n{1}", fun, block)
}

/// implementations of the VM's function return command
pub fn funret() -> String {
    format!(
        "// endframe = LCL\n\
         @LCL\n\
         D=M\n\
         @endframe\n\
         M=D\n\
         // retaddr = *(endframe - 5)\n\
         @endframe\n\
         D=M\n\
         @5\n\
         D=D-A\n\
         A=D\n\
         D=M\n\
         @retaddr\n\
         M=D\n\
         // *ARG = pop()\n\
         @SP\n\
         M=M-1\n\
         A=M\n\
         D=M\n\
         @ARG\n\
         A=M\n\
         M=D\n\
         // SP = ARG + 1\n\
         @ARG\n\
         D=M\n\
         D=D+1\n\
         @SP\n\
         M=D\n\
         // THAT = *(endframe - 1)\n\
         @endframe\n\
         D=M\n\
         @1\n\
         D=D-A\n\
         A=D\n\
         D=M\n\
         @THAT\n\
         M=D\n\
         // THIS = *(endframe - 2)\n\
         @endframe\n\
         D=M\n\
         @2\n\
         D=D-A\n\
         A=D\n\
         D=M\n\
         @THIS\n\
         M=D\n\
         // ARG = *(endframe - 3)\n\
         @endframe\n\
         D=M\n\
         @3\n\
         D=D-A\n\
         A=D\n\
         D=M\n\
         @ARG\n\
         M=D\n\
         // LCL = *(endframe - 4)\n\
         @endframe\n\
         D=M\n\
         @4\n\
         D=D-A\n\
         A=D\n\
         D=M\n\
         @LCL\n\
         M=D\n\
         // goto retaddr\n\
         @retaddr\n\
         A=M\n\
         0;JMP\n"
    )
}

/// implementations of the VM's bootstrap code
pub fn bootstrap() -> String {
    format!(
        "@256\n\
         D=A\n\
         @SP\n\
         M=D\n
         {}",
        funcall("Sys.init".to_string(), "0".to_string(), 0)
    )
}
