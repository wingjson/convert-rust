/*
 * @Date: 2024-02-23 16:02:47
 * @LastEditors: WWW
 * @LastEditTime: 2024-02-23 16:04:09
 * @FilePath: \ConvertTool\src-tauri\src\convert.rs
 */
use jni::objects::{JObject, JValue};
use crate::global::GLOBAL_JVM;

/**
 * @description: convert
 * @param {*} source target
 * @return {*}
 */
pub fn convert(source: String, target: String) -> jni::errors::Result<()> {

    let jvm_lock = GLOBAL_JVM.lock().unwrap();
    if let Some(ref jvm) = *jvm_lock {
        let mut env = jvm.attach_current_thread()?;
        // need find class by this type
        let class_name = env.find_class("org/convert/Main").unwrap();
        let method_name = "convert";
        let method_signature = "(Ljava/lang/String;Ljava/lang/String;)Z";
        let source_file_arg = env.new_string(source)?;
        let target_file_arg = env.new_string(target)?;
    
        //convert string to jvalue
        let binding_source = JObject::from(source_file_arg);
        let binding_target = JObject::from(target_file_arg);
        let source_file_jvalue = JValue::Object(&binding_source);
        let target_file_jvalue = JValue::Object(&binding_target);
    
        // call the method
        let result = env.call_static_method(
            class_name,
            method_name,
            method_signature,
            &[
                source_file_jvalue, 
                target_file_jvalue
            ]
        )?;
        if result.z()? {
            println!("convert success");
            // GLOBAL_DB.lock().unwrap().execute("insert into convert (source, target) values (?, ?)", &[&source, &target]).unwrap();
        }
    }
   
    Ok(())
}
