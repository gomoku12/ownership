fn main() {
    let s = String::from("hello");

    takes_ownership(s);

    //println!("{}", s); /* sは有効でないのでエラーが出る */

    /*s.push_str(", world!");

    println!("{}", s);
    
    let s1 = String::from("hello");
    let s2 = s1;
    println!("{}", s1);*/

    let s1 = gives_ownership();

    println!("{}", s1);

    let s2 = String::from("hello s2");

    println!("{}", s2);

    let s3 = take_and_give_back(s2);

    println!("{} from s3", s3);

    //println!("{}", s2); // s2はs3へムーブされるため，エラーが出る
    
}

fn gives_ownership() -> String {

    let some_string = String::from("hello from func: gives_ownership");

    some_string
}

fn takes_ownership(some_string: String){
    println!("{}", some_string);
}

fn take_and_give_back(a_string: String) -> String {

    a_string
}
