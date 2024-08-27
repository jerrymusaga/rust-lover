

/*
1. Each value has a variable that's it "owner"
2. A value can have only one owner at a time
3. If the owner goes out of scope, the value is cleaned up
*/

pub fn practice(){
    loop_ownership();
}

pub fn loop_ownership(){
    /* 
Fix the code below. By solving this problem you will be able to understand 
the change of ownership inside a loop 
*/ 
    let mut my_vec = vec![1, 2, 3, 4, 5];
    let mut temp;

    while !my_vec.is_empty() {
        temp = &my_vec; // Something wrong on this line
        println!("Elements in temporary vector are: {:?}", temp);


        if let Some(last_element) = my_vec.pop() { // pop() is used to remove an element from the vec
            println!("Popped element: {}", last_element);
        }
    }
}