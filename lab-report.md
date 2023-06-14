# 编译原理课程实践报告：用Rust实现的SysY编译器

北京大学 信息科学技术学院 1900012913 黄一凡


## 一、编译器概述

### 1.1 基本功能

本编译器基本具备如下功能：
1. 将 [SysY](https://pku-minic.github.io/online-doc/#/misc-app-ref/sysy-spec) 程序翻译成 [Koopa](https://pku-minic.github.io/online-doc/#/misc-app-ref/koopa) 中间表示：`-koopa`
2. 将 SysY 程序编译到 [RISC-V](https://pku-minic.github.io/online-doc/#/misc-app-ref/riscv-insts) ：`-riscv`

使用方式：`cargo run -- [-koopa | -riscv] <input> -o <output>`

### 1.2 主要特点

我开发的编译器的主要特点是**基于Rust语言**（我甚至是这学期刚接触的这门语言哈哈），以及**简单粗暴**（只有码量，没啥技巧）。


## 二、编译器设计

### 2.1 主要模块组成

编译器由2个层次构成：
* **应用层**：`src/main.rs` 负责解析命令行参数，并给出一些最简单的报错信息（指出错误发生在哪一阶段）
* **核心层**：`src/lib.rs` 及其他文件实现了编译器的核心功能，主要含3个模块：
	* `ast_generate`：词法分析，语法分析，语义分析，生成抽象语法树
	* `ir_generate`：遍历抽象语法树，得到 Koopa 中间表示（==文本形式==）；调用接口，将文字形式的 Koopa 转换成 [`koopa` crate](https://crates.io/crates/koopa) 定义的数据结构。
	* `target_generate`：遍历 Koopa 数据结构，得到 RISC-V 程序（文本形式）


### 2.2 主要数据结构

#### 2.2.1 从 SysY 到 Koopa

本编译器在 `ast_generate` 和 `ir_generate` 过程中最核心的数据结构是抽象语法树 AST 。在实际代码编写过程中，我参考了在线文档中对文法的定义，设计了一系列数据结构来表示 AST ；这些数据结构都在文件`src/ast_generate/ast.rs`中。这里，我大量使用了 Rust 的结构体 `struct` 和枚举类型 `enum` 来实现。例如，AST的树根 `CompUnit` 其实是一个 `CompUnitItem` 的 `Vec` ：

```rust
#[derive(Debug)]
pub struct CompUnit {
    pub items: Vec<CompUnitItem>,
}
```

而 `CompUnitItem` 可以是全局变量 / 常量定义，或者函数定义：

```rust
#[derive(Debug)]
pub enum CompUnitItem {
    GlobalDecl(GlobalDecl),
    FuncDef(FuncDef),
}
```

AST 的详细定义，见 `src/ast_generate/ast.rs`。

为了在 `ir_generate` 阶段中遍历 AST ，我在 `src/ir_generate/koopa_generate.rs` 中定义了 `KoopaTextGenerate` trait，并为 AST 的所有元素实现了它：

```rust
/// Run DFS on the AST and generate the Koopa text.
pub trait KoopaTextGenerate {
    /// Generate the Koopa text recursively.
    ///
    /// `lines` should always be empty when entering the method.
    fn generate(
        &self,
        lines: &mut String,
        scopes: &mut Scopes,
        tsm: &mut TempSymbolManager,
        nsc: &mut NamedSymbolCounter,
    ) -> Result<String, ()>;
}
```

其中 `lines` 是一次对 `generate` 的调用所生成的代码行；`scopes` 是作用域结构（下文会介绍）；`tsm` 是一个计数器，用来生成从不重复的 Koopa 临时符号；`nsc` 对每个变量名计数，用来生成从不重复的Koopa 具名符号。函数返回一个当前语句的左值（或空字符串，或出错）。

#### 2.2.2 从 Koopa 到 RISC-V

而在 `target_generate` 阶段，核心数据结构已经由 `koopa` crate 定义，见[官方文档](https://docs.rs/koopa/0.0.7/koopa/)。为了遍历这个结构，我在 `src/target_generate/riscv_generate.rs` 定义了 `RiscvGenerate` trait，并为 koopa 的所有核心数据结构实现了它：

```rust
/// Generate RISC-V code from the given Koopa object.
pub trait RiscvGenerate {
    /// The return type of the method `generate`.
    type Ret;

    /// Generate RISC-V code.
    ///
    /// `lines` should always be empty when entering the method.
    fn generate(
	    &self, 
	    lines: &mut String, 
	    cxt: &mut ProgramContext
	) -> Result<Self::Ret, ()>;
}
```

其中，`Ret`是返回值类型，可以为 unit 或者一个 `ValueLocation` （见下文）；这个associated type的灵感来自于 [pku-minic/kira-rs](https://github.com/pku-minic/kira-rs) ；`lines` 是这一次调用所生成的代码行，`cxt` 记录了一些遍历过程中必须使用的数据结构，具体内容在 `src/target_generate/context.rs`中定义：

```rust
/// Context information used during RISC-V assembly generation.
pub struct ProgramContext<'a> {
    /// The current program.
    pub program: &'a Program,
    /// The current function.
    pub func: Option<FunctionScanResult>,
    // The locations of the global values.
    pub global_values: HashMap<Value, ValueLocation>,
}
```

`program` 用来在翻译时获取一些 `FunctionData, ValueData` 等；`global_value` 记录了全局变量的位置 `ValueLocation` ；`func` 是当前函数在翻译前扫描得到的结果，其内容在 `src/target_generation/function_scan.rs` 中定义，字段含义见注释：

```rust
/// The result of function scanning.
pub struct FunctionScanResult {
    /// The handle of the function.
    ///
    /// Notice that `Function` has implemented the `Copy` trait!
    pub func: Function,
    /// The locations of the values in the function.
    ///
    /// During scanning, the addresses of all the variables are decided.
    /// Each of them has a **unique** location on the stack!
    /// Notice that `Value` has implemented the `Copy` trait!
    pub value_locations: HashMap<Value, ValueLocation>,
    /// Whether the `Value`'s `ValueLocation` contains a pointer to:
    /// 1. the data that a Koopa symbol refers to
    /// or
    /// 2. the data that a Koopa pointer points to
    /// rather than containing these data themselves.
    ///
    /// If true, we cannot load or store the `Value` directly.
    pub contain_pointer: HashMap<Value, bool>,
    /// The size of the stack frame.
    ///
    /// This value has been ceiled up to 16 bytes.
    pub stack_frame_size: usize,
    /// The location of the return address slot.
    ///
    /// `None` if the function does not call other functions.
    pub ra_slot_location: Option<ValueLocation>,
}
```

也就是说，函数局部变量的位置是在扫描阶段就分配好的（`value_locations`），而不是一边翻译一边分配的。

在 `target_generate` 阶段，另一个重要的概念是`ValueLocation`。它指明了某个量的位置，其内置的字符串字段通常可以直接拿来生成汇编代码，见 `src/target_generate/value_location.rs`：

```rust
/// The location of a value.
///
/// The `String` contained in the variants can be used 
/// directly in RISC-V instructions!
#[derive(Clone, PartialEq, Debug)]
pub enum ValueLocation {
    /// An Immediate value.
    Imm(String),
    /// Located in a register.
    Reg(String),
    /// Located on the stack frame.
    Stack(String),
    /// Global value
    Global(String),
    /// A placeholder.
    ///
    /// Used in the implementation of `FunctionData.generate(...)`,
    /// in order to put the value location 
    /// (returned by `cxt.get_value_location(Value)`) at a correct place.
    PlaceHolder(String),
    None,
}
```

### 2.3 主要设计考虑及算法选择

#### 2.3.1 符号表的设计考虑

在 `ir_generate` 阶段，需要使用符号表。符号表放在全体作用域结构中；全体作用域`Scopes`定义为：

```rust
#[allow(dead_code)]
pub struct Scopes {
    /// All the functions defined in the program.
    ///
    /// identifier -> koopa symbol name
    functions: HashMap<String, FunctionInfo>,
    /// Stacked symbol tables.
    ///
    /// identifier -> koopa symbol name / const value
    values: Vec<HashMap<String, SymbolTableValue>>,
    /// Contents of `values_buffer` will be inserted into 
    /// the scope entered next time.
    ///
    /// This field is used to put function parameters into the 
    /// symbol table of the function body.
    values_buffer: Vec<(String, SymbolTableValue)>,
    /// Stacked loop information.
    loops: Vec<LoopLabel>,
    /// The parameter list of the current function.
    cur_func_params: Vec<String>,
}
```

其中的 `values` 就是全体符号表构成的栈（其他字段含义见注释）。这里，用 `Vec` 来模拟栈：每进入一个新的作用域，就往 `values` 里面压入一个新符号表。`SymbolTableValue` 是符号表键值对中的“值”，指明这个符号是个变量、常量还是数组（处理各有不同）。

除了 `values` 以外，还有一个 `values_buffer`，用来暂存即将放入下一个进入的作用域的符号表里的键值对。这个结构是用来把函数参数放入函数 body 作用域的。

在 `target_generate` 阶段，2.2.2 章节提到的的 `ProgramContext` 起到了类似于符号表的作用，但不需要嵌套。这里就不重复叙述了。

#### 2.3.2 寄存器分配策略

全 扔 栈 上 ！

所有的运算均在寄存器 `t0, t1` 上进行；在少数的数组访问、以及立即数超界的情况下，可能会用到`t2, t3, t4` 来暂存中间结果。

#### 2.3.3 读取常量

虽然不是必要的，但我还是实现了常量的编译时求值。在 `ir_generate` 阶段，遇到常量定义时，不使用 `KoopaTextGenerate` trait 的 `genearte()` 方法来遍历表达式，而是调用这个 trait 所定义的方法，见 `src/ir_generate/exp_solve.rs` ： 

```rust
/// Solve the value of an expression.
pub trait ExpSolve {
    /// Evaluate the expression and return its value.
    ///
    /// This method is called when generating AST.
    fn solve(&self, scopes: &Scopes) -> Result<i32, ()>;
}
```

这个方法并不生成 Koopa 代码，而是直接求值；每个属于表达式类别的 AST 元素都实现了这个trait。

如此一来，就可以将常量及其对应值加入全局作用域对应的符号表里了；键值对的值是`SymbolTableValue::Const(String)`，字符串就是常量值的字面量。


## 三、编译器实现

### 3.1 各阶段实现细节

#### Lv1. main函数和Lv2. 初试目标代码生成

这个阶段的关键是建立项目结构，划分功能模块。上文已经介绍得很清晰啦。

#### Lv3. 表达式

只要严格按照在线文档里定义的文法来设计 AST，就能生成正确的“表达式执行树”，表达式优先级的问题自然就解决了！

以下是所有属于表达式类别的 AST 元素 （最终版，也包括了后面几个lab的要素）：

```rust
#[derive(Debug)]
pub struct ConstExp {
    pub exp: Exp,
}

#[derive(Debug)]
pub struct Exp {
    pub exp: LOrExp,
}

#[derive(Debug)]
pub enum LOrExp {
    LAnd(LAndExp),
    LOrLAnd(Box<LOrExp>, LAndExp),
}

#[derive(Debug)]
pub enum LAndExp {
    Eq(EqExp),
    LAndEq(Box<LAndExp>, EqExp),
}

#[derive(Debug)]
pub enum EqExp {
    Rel(RelExp),
    EqRel(Box<EqExp>, EqExpOp, RelExp),
}

#[derive(Debug)]
pub enum RelExp {
    Add(AddExp),
    RelAdd(Box<RelExp>, RelExpOp, AddExp),
}

#[derive(Debug)]
pub enum AddExp {
    Mul(MulExp),
    AddMul(Box<AddExp>, AddExpOp, MulExp),
}

#[derive(Debug)]
pub enum MulExp {
    Unary(UnaryExp),
    MulUnary(Box<MulExp>, MulExpOp, UnaryExp),
}

#[derive(Debug)]
pub enum UnaryExp {
    Primary(PrimaryExp),
    FuncCall(String, Vec<Exp>),
    Unary(UnaryExpOp, Box<UnaryExp>),
}

#[derive(Debug)]
pub enum PrimaryExp {
    Exp(Box<Exp>),
    LVal(LVal),
    Num(i32),
}

#[derive(Debug)]
pub struct LVal {
    pub ident: String,
    pub idx: Vec<Exp>,
}
```

值得注意的是 `Box<...>` 的使用。Rust 要求类型大小在编译时确定，因此不能有类型定义的“循环”出现（比如，`UnaryExp` 里面有另一个 `UnaryExp` ）。`Box<...>` 将对应数据放在堆上，拿一个指针指过去，解决了这个问题（因为指针大小确定）。上面的所有 `Box<...>` 都是必要的！

#### Lv4. 常量和变量

从这里开始，需要在 `ir_generate` 阶段引入符号表（但暂时不用嵌套符号表）。上文已经介绍了符号表的表示方式。

另外，常量定义应该在编译时计算出值，并从此用这个值来替代掉常量；变量定义则在运行时求值。处理时，需要分别使用 `solve()` 和 `generate()` 方法，见上文。例如，单个常量定义的实现如下：

```rust
impl KoopaTextGenerate for ConstDef {
    fn generate(
        &self,
        lines: &mut String,
        scopes: &mut Scopes,
        tsm: &mut TempSymbolManager,
        nsc: &mut NamedSymbolCounter,
    ) -> Result<String, ()> {
        if self.dims.is_empty() {
            // Constant scalars, both global and local.
            // No code line is generated, 
            // and the symbol will be replaced directly by its value.
            let init = self.init.generate(
	            &mut String::new(), 
	            scopes, tsm, nsc
	        )?; // Get the initial value.
            scopes.add_value(&self.ident, &init, true, None)?;
        } else { 
	        // constant arrays
	        ...
        }

        Ok(String::new())
    }
}
```

常量和变量的读取，查符号表即可。

RISC-V生成并不难，只需要在扫描阶段正确地给每个 Koopa 符号分配对应的位置 `ValueLocation` （栈上），就能顺利读写。

#### Lv5. 语句块和作用域

这部分的关键是 `ir_generate` 阶段的嵌套符号表，上文已经涉及。

下面是 AST 元素 `Block` 对 `KoopaTextGenerate` 的实现：
```rust
impl KoopaTextGenerate for Block {
    fn generate(
        &self,
        lines: &mut String,
        scopes: &mut Scopes,
        tsm: &mut TempSymbolManager,
        nsc: &mut NamedSymbolCounter,
    ) -> Result<String, ()> {
        scopes.enter()?;

        for item in self.items.iter() {
            let mut s = String::new();
            item.generate(&mut s, scopes, tsm, nsc)?;
            append_line(lines, &s);
        }

        scopes.exit();
        Ok(String::new())
    }
}
```

这里，`scopes.enter()` 和 `scopes.exit()` 分别往 `scopes.values` 这个嵌套符号表栈中加入和弹出一个新的表。

#### Lv6. if语句

关键是 `ast_generate` 阶段对 if 语句二义性的处理。这里，采用了最常规的、处理 dangling else problem 的方法，即每个 else 匹配其之前最近的 if，并在语法分析时分拆 `Stmt` 这个非终结符。lalrpop 文件中的相关部分为（包括了后面实验中的循环语句）

```rust
/// "else" is matched to the nearest "if" before it.
/// Hence, if these's a statement between "if" and "else", 
/// it must be a closed one (with "if" and "else" paired)!
/// Accordingly, we may remove ambiguity by splitting the definition of `Stmt`.
/// For further information, refer to 
/// https://en.wikipedia.org/wiki/Dangling_else .
///
///  Stmt ::= ClosedStmt
///         | OpenStmt
Stmt: Stmt = {
    <closed: ClosedStmt> => closed,
    <open: OpenStmt> => open,
}

/// ClosedStmt ::= "if" "(" Exp ")" ClosedStmt "else" ClosedStmt
///              | "while" "(" Exp ")" ClosedStmt
///              | OtherStmt
/// OtherStmt ::= LVal "=" Exp ";"
///              | [Exp] ";"
///              | Block
///              | "break" ";"
///              | "continue" ";"
///              | "return" [Exp] ";";
ClosedStmt: Stmt = {
    <lval: LVal> "=" <exp: Exp> ";" => Stmt::Assign(lval, exp),
    <exp: (Exp)?> ";" => Stmt::Exp(exp),
    <block: Block> => Stmt::Block(block),
    "if" "(" <cond: Exp> ")" <then: ClosedStmt> 
    "else" <otherwise: ClosedStmt> => {
        Stmt::If(cond, Box::new(then), Some(Box::new(otherwise)))
    },
    "while" "(" <cond: Exp> ")" <body: ClosedStmt> 
	    => Stmt::While(cond, Box::new(body)),
    "break" ";" => Stmt::Break,
    "continue" ";" => Stmt::Continue,
    "return" <exp: (Exp)?> ";" => Stmt::Return(exp),
}

/// OpenStmt ::= "if" "(" Exp ")" Stmt
///            | "if" "(" Exp ")" ClosedStmt "else" OpenStmt
///            | "while" "(" Exp ")" OpenStmt
OpenStmt: Stmt = {
    "if" "(" <cond: Exp> ")" <then: Stmt> 
	    => Stmt::If(cond, Box::new(then), None),
    "if" "(" <cond: Exp> ")" <then: ClosedStmt> 
	"else" <otherwise: OpenStmt> => {
        Stmt::If(cond, Box::new(then), Some(Box::new(otherwise)))
    },
    "while" "(" <cond: Exp> ")" <body: OpenStmt> 
	    => Stmt::While(cond, Box::new(body)),
}
```

注意：只需要改 lalrpop 的分析过程，不需要更改 AST 元素的定义！

从这里开始，Koopa 的每个函数就可能由==多个基本块==组成了。注意，==每个 Koopa 基本块的结尾必须有跳转或返回语句，而且这类语句必须位于基本块结束位置==。因此，处于函数中间位置的return语句后会开启一个新的基本块，但位于函数最后的return语句则不会开启新基本块。我的处理方法是，默认在return语句后产生一个基本块：

```rust
impl KoopaTextGenerate for Stmt {
    fn generate(
        &self,
        lines: &mut String,
        scopes: &mut Scopes,
        tsm: &mut TempSymbolManager,
        nsc: &mut NamedSymbolCounter,
    ) -> Result<String, ()> {
        match self {
            Self::Return(exp) => {
                // `ret` indicates the end of a basic block!!!
                let mut pre = String::new();
                if let Some(expression) = exp {
                    let ret = expression.generate(&mut pre, scopes, tsm, nsc)?;
                    append_line(&mut pre, &format!("  ret {}", ret));
                } else {
                    append_line(&mut pre, "  ret");
                }
                append_line(lines, &pre);
                let bb_label = nsc.inc_and_get_named_symbol("%after_return")?;
                append_line(lines, &format!("{}:", bb_label));
            }
            ......
        }

        Ok(String::new())
    }
}
```

到了整个函数定义的层面，如果函数体的最后一行是一个基本块名称，则把它删去。

```rust
impl KoopaTextGenerate for FuncDef {
    fn generate(
        &self,
        lines: &mut String,
        scopes: &mut Scopes,
        tsm: &mut TempSymbolManager,
        nsc: &mut NamedSymbolCounter,
    ) -> Result<String, ()> {
		 ......
		 
        // Return statements
        // 1. If there's no `ret` instruction in the function body, 
        //    we only need to add one at the last line.
        // 2. Only when the return type is `void` can the `ret` instruction 
        //    be omitted by the original function body.
        let Some(last_line) = body_text.split("\n").last() else {
            return Err(());
        };
        if last_line.contains("%after_return") {
            let Some(idx) = body_text.rfind("%after_return") else {
                return Err(());
            };
            body_text = body_text[..(idx-1)].to_string();
        } else if !last_line.contains("ret") {
            append_line(&mut body_text, "  ret");
        }

        ......
    }
}
```

#### Lv7. while语句

有了 lv6 的经验，这个部分确实不难；关键在于，如何在 `ir_generate` 阶段遇到 `break` 或 `continue`正确找到应该跳转的位置。为此，我在 `Scopes` 结构中添加了一个字段：

```rust
#[allow(dead_code)]
pub struct Scopes {
    ......
	
    /// Stacked loop information.
    loops: Vec<LoopLabel>,
	
    ......
}
```

这是个栈，记录了循环的嵌套结构；栈顶的就是当前循环。而每个循环由三个标签唯一确定：

```rust
/// The three labels defined for a `while` loop.
#[derive(Clone)]
pub struct LoopLabel {
    pub entry: String,
    pub body: String,
    pub end: String,
}
```

`break` 跳转到对应的 `end` 标签，而 `continue` 跳转到对应的 `entry` 标签就行。记得在跳转后开启一个新的基本块。

#### Lv8. 函数和全局变量

在 `ir_generation` 阶段，我们还需向 `Scopes` 中添加一个记录了所有函数的有关信息的表。函数的Koopa 符号、返回值和参数是否为数组都需要被记录。

```rust
/// Information about a function that will be used during Koopa text generation.
#[derive(Clone)]
pub struct FunctionInfo {
    pub symbol: String,
    pub return_void: bool,
    pub array_param: Vec<bool>,
    // there's no need to store detailed parameter types!!!
}

#[allow(dead_code)]
pub struct Scopes {
    /// All the functions defined in the program.
    ///
    /// identifier -> koopa symbol name
    functions: HashMap<String, FunctionInfo>,
    
    ......
}
```

在这个结构的帮助下，在 Koopa 文本中调用函数就没有困难了。

在 `target_generate` 阶段，难点在于函数栈帧的设置。正如上文所述，我的编译器会在翻译函数前先扫描一遍函数；经历这个过程后，函数的==栈帧大小、局部变量存储位置和返回值存储位置就完全确定==了。函数扫描的过程和结果的数据结构在 `src/target_generate/function_scan.rs` 中定义，章节 2.2.2 有做过展示。

读写当前函数参数或给即将调用的函数传参时，可以用 `src/target_generate/function_call.rs` 中的这个函数来找到参数位置：
```rust
/// Get the location of the i-th argument of a function.
/// 
/// If `call_another` is true, find the location of the parameter 
/// of another function being called now.
/// If false, find the location of the parameter of the current function.
pub fn function_arg_location(i: usize, stack_frame_size: usize, call_another: bool) -> ValueLocation {
    if i < 8 {
        ValueLocation::Reg(format!("a{}", i))
    } else {
        if call_another {
            ValueLocation::Stack(format!("{}(sp)", 4 * (i - 8)))
        } else {
            ValueLocation::Stack(
	            format!("{}(sp)", 4 * (i - 8) + stack_frame_size)
	        )
        }
        
    }
}
```

另一个小坑：虽然 Koopa 允许每个函数都有一个 `%entry` 基本块，但 RISC-V 不允许存在重名的标签。因此，干脆在 Koopa 的 `%entry` 基本块后加一个序号：`%entry_i`。

至于全局变量，需要在 `ir_generate` 阶段对 `VarDef` 做额外讨论，因为全局变量的初始值若有的话必是常量表达式，应该在编译时计算出来；全局常量则不必额外讨论（编译时计算出常量值，然后替代即可）。`target_generate` 阶段，对 `GlobalAlloc` 稍加处理，正确地初始化即可。

#### Lv9. 数组

非常难的一块内容。

`ir_generate`阶段需要考虑下面几点，相关工具函数都在`src/ir_generate/array_utils.rs` 中。

* 数组空间分配：用 `generate_alloc_dims()` 函数来递归地生成数组类型，如 `[[i32, 5], 3]`

* 数组初始化：用 `parse_const_array_initializer()` 或 `parse_var_array_initializer()`来解析不完整的初始化器，生成完整的初始化器（补0，参见[这个介绍](https://pku-minic.github.io/online-doc/#/lv9-array/nd-array?id=%e8%af%ad%e4%b9%89%e5%88%86%e6%9e%90)）。然后，用函数`full_initializer_to_global_lines()` 或 `full_initializer_to_local_lines` 来生成全局或局部数组初始化的代码行。

* 访问元素：用 `get_pointer_to_element_exp_idx()` 或 `get_pointer_to_element_int_idx()` 来生成访问元素的代码行（区别在于：访问过程中，下标存在表达式 or 全部是整数）。

`target_generate` 部分只需要对 `getelemptr` 语句加以翻译即可。利用 `koopa` crate 的接口，容易得到数组 base type的大小，从而生成计算偏移量的表达式。另外，需要用额外的结构来记录某个Koopa Value 所对应的 `ValueLocation` 处存储的是这个Value对应的数据本身还是指向数据的一个指针 （见 `FunctionScanResult` 的 `contain_pointer` 字段），并按照不同的方式来 load 和 store。

在 `target_generate` 阶段，当局部数组过大时，栈帧大小会超过 2048 bytes；此时，不能用 `addi` 来移动栈指针，也不能用 `offset(sp)` 的方式来访存 （因为offset过大）。对于移动栈指针的情况，做个 if 判断、必要时使用 `li` 和 `add` 来代替 `addi` 即可；对于访存的情况，我使用了下面的函数来把一个可能超界的访存转换成不会超界的情况，生成相应的代码行并返回新的访存 handle：

```rust
pub fn get_valid_address(
	addr: &str, temp_reg: &str, lines: &mut String
) -> String {
    let Some((base, offset)) = extract_base_and_offset(addr) else {
        return addr.to_string();
    };

    if offset < 2048 && offset >= -2048 {
        addr.to_string()
    } else {
        append_line(lines, &format!("  li {}, {}", temp_reg, offset));
        append_line(
	        lines, &format!("  add {}, {}, {}", temp_reg, base, temp_reg)
	    );
        format!("0({})", temp_reg)
    }
}
```

level 9.3 要求加入数组作为函数参数的功能。在 `ir_generate` 阶段，需要根据具体指针类型使用 `getptr` 来代替 `getelemptr`。具体来说，使用`getptr`的条件为：
* 该符号为当前函数的数组参数 （因为在SysY中，只有函数的数组参数可能采用指针形式传入，而非数组本身的形式）
* 对该符号的第一维度进行索引（对后续维度的索引，用`getelemptr`即可）。
将上述讨论融入 `get_pointer_to_element_exp_idx()` 和 `get_pointer_to_element_int_idx()`，就可以把它封装起来了。另外，一定要清楚分辨每个 Koopa 符号的类型，必要时做 load 来转换成目标类型。

### 3.2 工具软件介绍（若未使用特殊软件或库，则本部分可略过）

1. [`lalrpop` crate](https://crates.io/crates/lalrpop)：词法分析、语法分析、语义分析，将 SysY 程序转换成 AST。

2. [`koopa` crate](https://crates.io/crates/koopa)：将文本形式的 Koopa 中间表示转换成内存中的数据结构，并遍历它，生成目标汇编代码。


## 四、实习总结

### 4.1 收获和体会

三个月前，我连 Rust 都不会；现在，我居然能用 Rust 写出来一个简单的编译器辣！先给自己鼓个掌呗👏。

这个项目让我对编译原理理论课上所学内容的理解加深了。比如，此前我一直没彻底弄懂对 dangling else problem 的处理，直到写 level 6 自己真的遇到时才有了直观的体会。这种运用课堂所学解决实际问题、然后加深理解的过程，对我们这些忘性挺强的大学生而言是特别有益的！

除了“编译器”以外，这次经历对我而言是不可多得的大型项目构建、开发和维护经验。我自己属于那种乐于在项目初期花大功夫思考项目层级和模块结构的人，在想清楚前甚至不太愿意动手敲键盘。这种性格在这次经历里让我尝了很多甜头：整个过程中，我没有对项目架构进行大改，只是不断地增量式添加新内容；最后测试时，我能在发现问题后迅速定位，做出修改。在有了这次的正反馈后，我以后大改会延续这样的做事风格吧，哈哈！

最后，我想说说带给了我全新体验的 Rust 语言。在学习和使用 Rust 的过程中，我感受到这门语言的“精致”：必须考虑可能的错误情况，必须讨论一个 `enum` 的所有 variant；当代码写得“不好”时，连编译都过不了。而在我犯下错误时，Rust 的编译器总能精准定位，并给出修改提示。这种“掌控感”和“安全感”是我以前编程时所没有感受过的。很高兴能借这次实践的机会认知这样一门有意思的语言！

### 4.2 学习过程中的难点，以及对实习过程和内容的建议

level 9 数组和多维数组真的很难很难…… 能否在在线文档里多加一些相关的提示？

### 4.3 对老师讲解内容与方式的建议

现在挺好！
