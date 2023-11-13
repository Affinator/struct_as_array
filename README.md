A Rust library that allows you to represent the structure as an array.                      
Library works only with named structs whose fields have the same type.                      
                                                                                            
# Examples                                                                                  
Basic usage:                                                                                
```                                                                                         
use struct_as_array::*;                                                                     
                                                                                            
#[derive(AsArray)]                                                                          
struct TestStruct {                                                                         
    t1: i32,                                                                                
    t2: i32,                                                                                
    t3: i32,                                                                                
}                                                                                           
                                                                                            
let t = TestStruct {                                                                        
    t1: 0,                                                                                  
    t2: 1,                                                                                  
    t3: 2,                                                                                  
};                                                                                          
                                                                                            
assert_eq!(t.as_array(), [&0, &1, &2]);                                                     
```                                                                                         
                                                                                            
Using as an iterator:                                                                       
                                                                                            
```                                                                                         
use struct_as_array::*;                                                                     
                                                                                            
#[derive(AsArray)]                                                                          
struct TestStruct {                                                                         
    t1: i32,                                                                                
    t2: i32,                                                                                
    t3: i32,                                                                                
}                                                                                           
                                                                                            
let t = TestStruct {                                                                        
    t1: 0,                                                                                  
    t2: 1,                                                                                  
    t3: 2,                                                                                  
};                                                                                          
                                                                                            
for i in t.as_array() {                                                                     
    println!("{}", i);                                                                      
}                                                                                           
```                                                                                         
