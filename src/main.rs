// fn main() {
//     let v1=vec![10,20,30,40,50];
//     println!("{:?}",v1);
// }

// PUS OR POP IN VECTORS
// fn main(){
//     let mut v1:Vec<i32>=Vec::new();
//     v1.push(30);
//     v1.push(30);
//     v1.push(40);
//     v1.push(50);
//     v1.push(60);
//     println!("{:?}",v1);

//     v1.pop();
//     println!("{:?}",v1);

// }

// Accessing values
// fn main(){


// let v1=vec![10,20,30,40,50];
// // let e=v1[2];
// // let e=&v1[4];
// let e=v1.get(4);

// match e{
//     Some(val) => println!("{:?}",val),
//     None => println!("NO Value"),
// }

// }

// Looing in vectors
// fn main(){


// let v=vec!["Ahsan","Ali","Tanzeel"];
// for i in &v{
//     println!("{:#?}",i);
// }
// println!("{:?}",v[0]);
// }

// fn main(){
//     let mut v=vec![10,20,30,40];
//     for i in & mut v{
//         *i+=20;
//         println!("{:#?}",i);
//     }
// }

// ENUMS AND VECTORS
// #[derive(Debug)]
// enum Spread{
//     Int(i32),
//     Float(f64),
//     Text(String),
// }

// fn main(){
//     let row = vec![Spread::Int(333),Spread::Float(32.9),Spread::Text(String::from("This is Syed Ahsan"))];
//     println!("{:#?}",row);
// }

// Converting STR to String 
// fn main(){


// let s1="SYED AHSAN";
// let s2=s1.to_string();
// println!("{}",s2);

// }

// String PUSH AND POP
// fn main(){
//     let mut s1=String::from("Ahsan");
//     s1.push_str("Tipu");
//     println!("{}",s1);

//     s1.push('A');
//     println!("{}",s1);
// }

// CONCATENATION IN STRINGS
// fn main(){
//     let s1="AHSAN";
//     let s2="Tipu";
//     let s3=s1+&s2;
//     println!("{:?}",s3);
// }

// fn main(){
//     let s1="Syed";
//     let s2="Ahsan";
//     let s3="Ali";
//     let s=format!("{} {} {}",s1,s2,s3);
//     println!("{}",s);
// }

// INDEX AT STRING
// fn main(){


// let s1="Syed AHsan Ali";
// let indexa=&s1[0..7];
// println!("{}",indexa);
// }

// Bytes language difference in strings
// fn main(){


// let s1=String::from("long live").len();
// let s2=String::from("larga vida").len();
// println!("{}",s1);
// println!("{}",s2);
// } 

// ITERATING ON STRINGS
// fn main(){
//     for e in "Syed Ahsan ALi".chars(){
//         println!("{}",e);
//     }
// }

// ITERATING STRINGS USING BYTES
// fn main(){
//     for e in "Syed Ahsan ALi".bytes(){
//                 println!("{}",e);
//             }
// }

// HASHMAPS===============
// use std::collections::HashMap;
// fn main(){


// let mut map=HashMap::new();
// map.insert(String::from("Syed Ahsan Ali"),1);
// map.insert(String::from("M.Ali Khan"),2);
// map.insert(String::from("Syed Ahsan Ali"),3);
// println!("{:#?}",map);
// }

// // HashMap Construction============
// use std::collections::HashMap;
// fn main(){
//     let color=vec![String::from("Black"),String::from("Blue")];
//     let key_value=vec![10,20];
//     let  map:HashMap<_,_> =color.iter().zip(key_value.iter()).collect();
// println!("{:?}",map);
// }

// use std::collections::HashMap;
// fn main(){
//     let mut map=HashMap::new();
//     map.insert(String::from("Ahasn"),1);
//     map.insert(String::from("Tipu"),2);

//     for (key,value) in &map{
//         println!("{},{}",key,value);
//     }
// }

// ENTRY 

// use std::collections::HashMap;
// fn main(){
//     let mut map=HashMap::new();
//     map.entry(String::from("Ahasn")).or_insert(20);
//         map.entry(String::from("Tipu")).or_insert(40);
//     println!("{:?}",map);
// }

// use std::collections::HashMap;
// fn main(){
//     let fna=String::from("Blue");
//     let f=20;
//     let mut map=HashMap::new();
//     map.insert(fna,f);
//     println!("{:?}",map);
//     let ac=String::from("Yellow");
//     let z=map.get(&ac);
//     println!("{:?}",z);
// }

// // WORDS COUNTER=====================
// use std::collections::HashMap;
// fn main(){
//     let my_string="Hellow World Beautiful World Virtual World";
//     let mut map=HashMap::new();
//     for word in my_string.split_whitespace(){
//         let count=map.entry(word).or_insert(0);
//         *count+=1;
//     }
//     println!("{:#?}",map);
// }