use quickjs_runtime::builder::QuickJsRuntimeBuilder;
use quickjs_runtime::jsutils::Script;
use quickjs_runtime::values::JsValueFacade;
#[tokio::main]
async fn main() {
    let context = QuickJsRuntimeBuilder::new().build();
    let result = context.eval_sync(None, Script::new("file://main.js", "(function() { return [1,2,3,4]; })()")).unwrap();
    println!("{:?}", result.to_serde_value().await.unwrap());

    match result {
         JsValueFacade::Array { val: arr } => {
            println!("{:?}", arr);
        }
        JsValueFacade::JsArray { cached_array: str } => {
            println!("{:?}", str.get_array().await.unwrap());
        }
        _ => {}
    };
}