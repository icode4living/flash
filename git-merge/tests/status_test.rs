use utils::status::*;
pub fn add(x: i32, y: i32)-> i32{
    x+y
}


   #[test]
   fn add_test(){
assert_eq!(add(2,2),4);
   }
