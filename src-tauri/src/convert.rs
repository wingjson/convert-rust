/*
 * @Date: 2024-02-23 16:02:47
 * @LastEditors: WWW
 * @LastEditTime: 2024-03-02 18:02:37
 * @FilePath: \convert-rust\src-tauri\src\convert.rs
 */
use jni::{objects::{JObject, JValue}, JavaVM, JNIVersion};
use crate::global::GLOBAL_JVM;
use std::env;
/**
 * @description: convert
 * @param {*} source target
 * @return {*}
 */
pub fn convert(source: String, target: String) -> jni::errors::Result<String> {

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
            Ok("success".to_string())
        }else{
            Ok("failed".to_string())
        }
    }else{
        Ok("env failed".to_string())
    }
   
    
}


pub fn convert_test(jvm:JavaVM,source: String, target: String) -> jni::errors::Result<String> {

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
            Ok("success".to_string())
        }else{
            Ok("failed".to_string())
        }
   
    
}
#[test]
fn test() {
                env::set_var("JAVA_HOME", "E:/Code/rust/convert-rust/src-tauri/libs/env/bin");
                let class_path_option = format!("-Djava.class.path={}","E:/Code/rust/convert-rust/src-tauri/libs/file-slim.jar");
                let jvm_args = jni::InitArgsBuilder::new()
                    .version(JNIVersion::V8)
                    .option(&class_path_option)
                    .build().unwrap();

                let jvm = JavaVM::new(jvm_args).unwrap();
                convert_test(jvm, "E:/Code/rust/convert-rust/1.pdf".to_string(), "E:/Code/rust/convert-rust/1.docx".to_string()).unwrap();
}