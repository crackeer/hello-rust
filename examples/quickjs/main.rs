use quick_js::{Context, JsValue};

fn main() {
    // 创建 JS 上下文
    let context = Context::new().unwrap();

    // 执行一段 JS 表达式
    let value = context.eval("1 + 2").unwrap();
    assert_eq!(value, JsValue::Int(3));

    // 执行一段 JS 代码并返回字符串
    let value = context
        .eval_as::<String>("var x = 100 + 250; x.toString()")
        .unwrap();
    assert_eq!(&value, "350");

    // 注册 Rust 函数到 JS 中
    context.add_callback("add", |a: i32, b: i32| a + b).unwrap();

    // 在 JS 中调用 Rust 函数
    context
        .eval(
            r#"
            const result = add(10, 20);
            console.log("Result from Rust:", result);
        "#,
        )
        .unwrap();
}