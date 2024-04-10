// lifetimes2.rs
//
// So if the compiler is just validating the references passed to the annotated
// parameters and the return type, what do we need to change?
//
// Execute `rustlings hint lifetimes2` or use the `hint` watch subcommand for a
// hint.

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

//方法一：去掉main方法里面的子作用域
//方法二：把打印放到子作用域中去
//方法三：把string2的声明向上挪动，放到子作用域外
fn main() {
    let string1 = String::from("long string is long");
    let result;

    let string2 = String::from("xyz");
    {
        result = longest(string1.as_str(), string2.as_str());
    }
        println!("The longest string is '{}'", result);
}
