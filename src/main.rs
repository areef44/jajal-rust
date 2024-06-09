// memanggil mod dari file sebelah
mod first;
mod second;
mod model;
mod third;



// implement use
use first::say_hello;
use second::say_hello as say_hello_second;

#[test]
fn test_use() {
    say_hello();
    say_hello_second();
    first::second::third::say_hello();
}

use model::User;

//memanggil module yang sudah dijadikan public
#[test]
fn test_module() {
    let user: User = User{
        first_name: String::from("Arif"),
        last_name: String::from("Muhammad"),
        username: String::from("areef44"),
        email: String::from("areef44@gmail.com"),
        age: 20
    };
    user.say_hello("Joko");
}

fn main() {
    println!("Hello, world!");
}

// unit test
#[test]
fn hello_test(){
    println!("Hello Test")
}

// variable || gunakan let mut agar isi variable dapat berubah
#[test]
fn test_variable() {
    let name= "Muhammad Arif";
    println!("Hello {}", name);
}

#[test]
fn test_mutable() {
    let mut name= "Muhammad Arif";
    println!("Hello {}", name);

    name = "Ariel Peterpan";
    println!("Hello {}", name);

}

#[test]
fn static_typing() {
    let name= "Muhammad Arif";
    println!("Hello {}", name);

    // name = 10;
    println!("Hello {}", name);

}

#[test]
fn shadowing() {
    let name= "Muhammad Arif";
    println!("Hello {}", name);

    let name = 10;
    println!("Hello {}", name);

}

#[test]
fn explicit(){
    let age = 20;
    print!("{}", age);
}

#[test]
fn number(){
    let a:i32=10;
    println!("{}", a);

    let b:f64 = 10.5;
    println!("{}", b);

}

#[test]
fn number_conversion(){
    let a:i8=10;
    println!("{}", a);

    let b:i16 = a as i16;
    println!("{}", b);

    let c:i32 = b as i32;
    println!("{}", c);

    // contoh integer overflow
    let d:i64 = 1000000000;
    let e:i8 = d as i8;
    println!("{}", e);
}

#[test]
fn number_operations(){
    let a = 10;
    let b = 10;

    let c = a * b;
    println!("{}", c);

    let d = a / b;
    println!("{}", d);

    let e = a + b;
    println!("{}", e);
    
    let f = a - b;
    println!("{}", f);
}

#[test]
fn augmented_assignment(){
    let mut a = 10;
    println!("{}", a);

    a += 10;
    println!("{}", a);

    a -= 10;
    println!("{}", a);
}

#[test]
fn boolean(){
    let a = true;
    let b: bool = false;

    println!("{}",a);
    println!("{}",b);

}

#[test]
fn comparison(){
    let a = 10;
    let b = 20;
    let result: bool = a < b;
    println!("{}", result)
}

#[test]
fn boolean_operator(){
    let absen = 70;
    let nilai_akhir = 80;

    let lulus_absen = absen >= 75;
    let lulus_nilai_akhir = nilai_akhir >= 75;

    let lulus = lulus_absen && lulus_nilai_akhir;
    println!("{}", lulus);

}

#[test]
fn char_type(){
    let char1 = 'a';
    let char2 = 'b';

    println!("{} {}", char1, char2);

}

#[test]
fn tuple(){
    let data: (i32, f64, bool) = (8, 10.5, true);

    println!("{:?}",data);

    // let a = data.0;
    // let b = data.1;
    // let c = data.2;

    let (a, b, c) = data;
    println!("{} {} {}", a , b , c);
}

#[test]
fn mutable_tuple(){
    let mut data: (i32, f64, bool) = (8, 10.5, true);

    println!("{:?}",data);

    // let a = data.0;
    // let b = data.1;
    // let c = data.2;

    let (a, b, c) = data;
    println!("{} {} {}", a , b , c);

    data.0 = 20;
    data.1= 20.5;
    data.2= false;

    println!("{:?}", data)
}

#[test]
fn unit(){
    println!("hello");
}

#[test]
fn test_unit(){
    let result: () = unit();
    println!("{:?}", result);

    let test: () = ();
    println!("{:?}", test);
}

#[test]
fn array(){
    let array: [i32;5] = [1,2,3,4,5];
    println!("{:?}", array);

    let a = array[0];
    let b = array[1];

    println!("{}{}", a, b);

}

#[test]
fn mutable_array(){
    let mut array: [i32;5] = [1,2,3,4,5];
    println!("{:?}", array);

    let a = array[0];
    let b = array[1];

    println!("{}{}", a, b);

    array[0] = 10;
    array[1] = 20;
    println!("{:?}", array);

    let length: usize = array.len();
    print!("{}", length);
}

#[test]
fn two_dimentional_array() {
    let matrix: [[i32; 3]; 2] = [
        [1, 2,3],
        [4, 5,6]
    ];

    println!("{:?}", matrix);
    println!("{:?}", matrix[0]);
    println!("{}", matrix[0][0]);
    println!("{}", matrix[0][1]);
    println!("{}", matrix[0][2]);
    println!("{:?}", matrix[1]);
    println!("{}", matrix[1][0]);
    println!("{}", matrix[1][1]);
    println!("{}", matrix[1][2]);
    
}

const MAXIMUM: i32 = 100;
#[test]
fn constant() {
    const MINIMUM: i32 = 0;
    println!("{} {}", MINIMUM, MAXIMUM);
}

#[test]
fn variable_scope() {
    let variable = 1;

    { // inner scope
        println!("inner variable: {}", variable);
        let variable2 = 2;
        println!("inner variable2: {}", variable2);
    }

    print!("inner variable2: {}", variable); //error
}

#[test]
fn stack_heap() {
    function_a();
    function_b();
}

fn function_a(){
    let a = 10;
    let b= String::from("Arif");
    println!("{} {}", a, b);
}

fn function_b() {
    let a = 10;
    let b= String::from("Muhammad");
    println!("{} {}", a, b);
}

#[test]
fn string_slice() {
    let name = " Muhammad Arif ";
    let trim = name.trim();

    println!("{}", name);
    println!("{}", trim);
}

#[test]
fn string_type(){
    let mut name: String = String::from("Arif");
    name.push_str(" Muhammad");

    println!("{}", name);

    let new_name = name.replace("Arif", "Barjo");
    println!("{}", name);
    println!("{}", new_name);

}

#[test]
fn ownership_rules(){
    // a tidak bisa diakses disini, belum dideklarasikan
    let a = 10; // a bisa diakses mulai dari sini

    { // b tidak bisa diakses disini, belum dideklarasikan
        let b = 20; // b bisa diakses mulai disini
        println!("{}", b);
    } // scope b selesai, b dihapus, b tidak bisa diakses lagi

    println!("{}", a);

} // scope a selesai, a dihapus, a tidak bisa diakses lagi

#[test]
fn data_copy(){
    let a=10;
    let b= a;

    println!("{} {}", a, b);
}

#[test]
fn ownership_movement(){
    let name1 = String::from("Arif");

    println!("{}", name1);

    //Ownership dari name1 dipindahkan ke name2
    let name2 = name1;
    // name1 tidak bisa diakses kesini
    println!("{}", name2);
}

#[test]
fn clone(){
    let name1 = String::from("Arif");
    // membuat data tiruan yang sama
    let name2= name1.clone();

    println!("{} {}", name1, name2);
}

#[test]
fn if_expression() {
    let value = 2;

    if value >= 8 {
        println!("Good");
    } else if value >= 6{
        println!("Not Bad");
    } else if value >= 3 {
        println!("Bad");
    } else {
        println!("Very Bad");
    }
}

#[test]
fn let_statement() {
    let value = 9;
    let result: &str ;

    if value >= 8 {
        result = "Good";
    } else if value >= 6{
        result = "Not Bad";
    } else if value >= 3 {
        result = "Bad";
    } else {
        result = "Very Bad";
    }

    println!("{}", result);
}

#[test]
fn let_statement_optional() {
    let value = 6;
    let result: &str = if value >= 8 {
        "Good"
    } else if value >= 6{
        "Not Bad"
    } else if value >= 3 {
        "Bad"
    } else {
        "Very Bad"
    };

    println!("{}", result);
}

#[test]
fn loop_expression(){

    let mut counter = 0;

    loop {
        counter += 1;

        if counter > 10 {
            break;
        } else if counter % 2 == 0 {
            continue;
        }
        println!("Counter: {}", counter);

    }
}

#[test]
fn loop_return_value() {
    let mut counter = 0;
    let result = loop {
        counter += 1;
        if counter > 10 {
            break counter * 2; 
        }
    };

    println!("Result: {}", result);
}

#[test]
fn loop_label(){
    let mut number = 1;
    'outer: loop {
        let mut i = 1;
        loop {
            if number > 10 {
                break 'outer;
            }

            println!("{} x {} = {}", number, i, number * 1 );
            i += 1;
            if i > 10 {
                break;
            }
        }

        number += 1;
    }
}

#[test]
fn while_loop() {
    let mut counter = 0;
    while counter <= 10 {
        if counter % 2 == 0 {
            println!("Counter: {}", counter);
        }
        counter += 1;
    }
}

#[test]
fn array_iteration() {
    let array: [&str; 5] = ["A", "B", "C", "D", "E"];
    let mut index = 0;

    while index < array.len() {
        println!("Value :  {}", array[index]);
        index += 1;
    }
}

#[test]
fn array_iteration_for() {
    let array: [&str; 5] = ["A", "B", "C", "D", "E"];
    
    for value in array {
        println!("Value :  {}", value);
    }
}


#[test]
fn range() {
    let array: [&str; 5] = ["A", "B", "C", "D", "E"];

    let range = 0..5;
    println!("Start: {}", range.start);
    println!("End: {}", range.end);

    for value in range {
        println!("Value :  {}", array[value]);
    }

}

#[test]
fn range_inclusive() {
    let array: [&str; 5] = ["A", "B", "C", "D", "E"];

    let range = 0..=4;
    println!("Start: {}", range.start());
    println!("End: {}", range.end());

    for value in range {
        println!("Value :  {}", array[value]);
    }

}

// fn say_hello(){
//     println!("Hello");
// }

#[test]
fn test_say_hello(){
    say_hello();
    say_hello();
    say_hello();

}

fn say_goodbye(first_name: &str, last_name: &str){
    println!("Goodbye {} {}", first_name, last_name);
}

#[test]
fn test_say_goodbye(){
    say_goodbye("Muhammad", "Arif");
    say_goodbye("Jaka", "Wihdada");
    say_goodbye("Gibran", "Rakabooming");
}

fn factorial_loop(n: i32) -> i32 {
    if n < 1 {
        return 0;
    }

    let mut result = 1;

    for i in 1..=n {
        result *= i
    }

    result
}

#[test]
fn test_factorial_loop() {
    let result = factorial_loop(5);
    println!("{}", result);
    let result = factorial_loop(-10);
    println!("{}", result);
}

fn print_text(value: String, times: u32) {
    if times == 0 {
        return;
    } else {
        println!("{}", value);
    }

    print_text(value, times -1);

}

#[test]
fn test_print_text() {
    print_text(String::from("Arif"), 10)
}

fn factorial_recursive(n: u32) -> u32 {
    if n <= 1 {
        return 1;
    }

    n * factorial_recursive(n -1)
}

#[test]
fn test_factorial_recursive() {
    let result = factorial_recursive(5);
    println!("{}", result);
}

fn print_number(number: i32){
    println!("number {}", number);
}

fn hi(name: String){
    print!("name {}", name);
}

#[test]
fn test_hi() {
    let number = 10;
    print_number(number);
    println!("{}", number);

    let name = String::from("Arif");
    hi(name);
    // println!("{}", name);
}

fn full_name(first_name: String, last_name: String) -> String {
    format!("{} {}", first_name, last_name)
}

#[test]
fn test_full_name() {
    let first_name = String::from("Arif");
    let last_name = String::from("Muhammad");
    let full_name = full_name(first_name, last_name);

    println!("{}", full_name);
    // println!("{}", first_name);
    // println!("{}", last_name);
}

fn full_name_ownership(first_name: String, last_name: String) -> (String,String,String) {
    let full_name = format!("{} {}", first_name, last_name);

    (first_name, last_name, full_name)
}


#[test]
fn test_full_name_ownership() {
    let first_name = String::from("Arif");
    let last_name = String::from("Muhammad");

    let (first_name, last_name, name) = full_name_ownership(first_name, last_name);
    println!("{}", name);
    println!("{}", first_name);
    println!("{}", last_name);
}


fn full_name_reference(first_name: &String, last_name: &String) -> String {
    format!("{} {}", first_name, last_name)
}

#[test]
fn test_full_name_reference() {
    let first_name = String::from("Arif");
    let last_name = String::from("Muhammad");

    let full_name = full_name_reference(&first_name, &last_name);

    println!("{}", full_name);
    println!("{}", first_name);
    println!("{}", last_name);
}

fn change_value(value: &mut String){
    value.push_str("Test");
}

#[test]
fn test_change_value() {
    let mut value = String::from("Arif");
    change_value(&mut value);
    println!("{}", value);
}

#[test]
fn slice_reference() {
    let array: [i32; 10] = [1,2,3,4,5,6,7,8,9,10];

    let slice1: &[i32] = &array[..];
    println!("{:?}", slice1);

    let slice2: &[i32] = &array[0..5];
    println!("{:?}", slice2);

    let slice3: &[i32] = &array[5..];
    println!("{:?}", slice3);

}

#[test]
fn string_slice_example() {
    let name: String = String::from("Arif Muhammad");
    let first_name: &str = &name[0..4];
    println!("{}", first_name);

    let last_name: &str = &name[5..];
    println!("{}", last_name);
}

struct Person {
    first_name: String,
    last_name: String,
    age: u8,
}

// membuat method
impl Person {
    fn say_hello(&self, name: &str){
        println!("Hello {}, my name is {}", name, self.first_name);
    }
}

fn print_person(person: &Person){
    println!("{}", person.first_name);
    println!("{}", person.last_name);
    println!("{}", person.age);
}

#[test]
fn struct_person(){
    let person: Person = Person {
        first_name: String::from("Muhammad"),
        last_name: String::from("Arif"),
        age: 35,
    };

    println!("{}", person.first_name);
    println!("{}", person.last_name);
    println!("{}", person.age);

    print_person(&person);

}

#[test]
fn init_shorthand(){

    let first_name = String::from("Arif");
    let last_name = String::from("Muhammad");


    let person: Person = Person {
        first_name,
        last_name,
        age: 35,
    };

    // println!("{}", first_name); --> tidak bisa dipindah karena ownership dipindah

    print_person(&person);

    // update syntac struct
    // let person2 = Person{..person};

    let person2 = Person{
        first_name: person.first_name.clone(),
        last_name: person.last_name.clone(),
        ..person
    };

    print_person(&person2)

}


struct GeoPoint(f64, f64);
//associated function
impl GeoPoint {
    fn new(long: f64, lat: f64) -> GeoPoint {
        GeoPoint(long, lat)
    }
}

#[test]
fn tuple_struct() {
    let geo_point = GeoPoint(-6.2313213, 106.898098);
    println!("lat  : {}", geo_point.0);
    println!("long : {}", geo_point.1);

}

struct Kosong;

#[test]
fn test_nothing() {
    // kasih _ kalo variable tidak ingin digunakan dan ga diberi warning
    let _kosong1: Kosong = Kosong;
    let _kosong2: Kosong = Kosong{};
}

#[test]
fn test_method(){
    let person = Person{
        first_name: String::from("Arif"),
        last_name: String::from("Muhammad"),
        age: 35,
    };

    person.say_hello("Joko");
}

//associated function
#[test]
fn test_method_new() {
    let geo_point: GeoPoint = GeoPoint::new(-6.2313213, 106.898098);
    println!("lat : {}", geo_point.0);
    println!("long : {}", geo_point.1);
}

enum Level {
    Regular,
    Premium,
    Platinum,
}

#[test]
fn test_enum() {
    let level: Level = Level::Regular;

    // pattern matching enum
    match level {
        Level::Regular => {
            println!("Regular")
        }
        Level::Premium => {
            println!("Premium")
        }
        Level::Platinum => {
            println!("Platinum")
        }
    }
}

enum Payment {
    CreditCard(String),
    BankTransfer(String, String),
    EWallet(String, String),
}

impl Payment {
    // destructuring enum data patterns
    fn pay(&self, amount: u32){
        match self {
            Payment::CreditCard(number) => {
                println!("Paying with credit card {} amount {}", number, amount);
            }
            Payment::BankTransfer(bank, number) => {
                println!("Paying with bank transfer {} {} amount {}", bank, number, amount);
            }
            Payment::EWallet(wallet, number) => {
                println!("Paying with ewallet {} {} amount {}", wallet, number, amount);
            }
        }
    }
}

#[test]
fn test_payment() {
    let _payment1: Payment = Payment::CreditCard(String::from("BCA"));
    let _payment2: Payment = Payment::BankTransfer(String::from("BCA"), String::from("213213213"));
    let _payment3: Payment = Payment::EWallet(String::from("Dana"), String::from("081334343545"));

}

#[test]
fn test_payment_method() {
    let _payment1: Payment = Payment::CreditCard(String::from("34234324324"));
    _payment1.pay(20000);
    let _payment2: Payment = Payment::BankTransfer(String::from("BCA"), String::from("34234234324"));
    _payment2.pay(1000);
    let _payment3: Payment = Payment::EWallet(String::from("OVO"), String::from("34234234324"));
    _payment2.pay(500);
}

#[test]
fn test_match_value(){
    
    let name = "tarjo";

    match name {
        "Arif" | "tarjo" => {
            println!("Hello Guys");
        }
        "Joko" => {
            println!("Hello Joko");
        }
        other => {
            println!("Hello {}", other);
        }
    }
}

#[test]
fn test_range_patterns(){
    
    let value = 100;

    match value {
        75..=100 => {
            println!("A");
        }
        50..=74 => {
            println!("B");
        }
        25..=49 => {
            println!("C");
        }
        other => {
            println!("Tidak Lulus {}", other);
        }
    }
}

#[test]
fn test_struct_patterns(){
    
    let point = GeoPoint(0.0, 106.0455435);

    match point {
        GeoPoint(long, 0.0) => {
            println!("long: {}", long);
        }
        GeoPoint(0.0, lat) => {
            println!("lat: {}", lat);
        }
        GeoPoint(long, lat) => {
            println!("long: {} lat: {}", long, lat);
        }
    }

    let person = Person{
        first_name: String::from("Arif"),
        last_name: String::from("Muhammad"),
        age: 35,
    };

    match person {
        Person{first_name, last_name, ..} => {
            println!("{} {}", first_name, last_name);
        }
    }
}

#[test]
fn test_geopoint_ignoring(){
    
    let point = GeoPoint(0.0, 106.0455435);

    match point {
        GeoPoint(_, long) => {
            println!("long: {}", long);
        }
    }
}

#[test]
fn test_range_patterns_ignoring(){
    
    let value = 24;

    match value {
        75..=100 => {
            println!("A");
        }
        50..=74 => {
            println!("B");
        }
        25..=49 => {
            println!("C");
        }
        _ => {
            println!("Tidak Lulus");
        }
    }
}

#[test]
fn test_match_expression() {
    let value: i32 = 2;
    let result: &str = match value {
        0 => "nol",
        1 => "satu",
        2 => "dua",
        _ => "invalid"
    };
    println!("{}", result);
}


// Implement Type Alias
type Age = u8;
type IdentityNumber = String;

struct Customer {
    id: IdentityNumber,
    name: String,
    age: Age,
}

type Pelanggan = Customer;

#[test]
fn test_customer() {
    let customer = Customer {
        id: String::from("003"),
        name: String::from("Arif"),
        age: 20,
    };

    println!("{} {} {}", customer.id, customer.name, customer.age);
}

 // implementasi trait
trait CanSayHello {
    fn hello(&self) -> String {
        String::from("Hello")
    }
    fn say_hello(&self) -> String;
    fn say_hello_to(&self, name: &str) -> String;
}

impl CanSayHello for Person {
    fn say_hello(&self) -> String{
        format!("Hello, my name is {}", self.first_name)
    }
    fn say_hello_to(&self, name: &str) -> String{
        format!("Hello {}, my name is {}", name, self.first_name)
    }
}

fn say_hello_trait(value: &impl CanSayHello){
    println!("{}", value.say_hello());
}


trait  CanSayGoodBye {
    fn good_bye(&self) -> String;
    fn good_bye_to(&self, name: &str) -> String;
}

impl CanSayGoodBye for Person {
    fn good_bye(&self) -> String {
        format!("Good bye, my name is {}", self.first_name)
    }

    fn good_bye_to(&self, name: &str) -> String {
        format!("Good bye, {} my name is {}", name, self.first_name)
    }
}

fn hello_and_goodbye(value: &(impl CanSayHello + CanSayGoodBye)){
    println!("{}", value.say_hello());
    println!("{}", value.good_bye());
}

#[test]
fn test_trait() {
    let person: Person = Person{
        first_name: String::from("Arif"),
        last_name: String:: from("Muhammad"),
        age: 30,
    };

    say_hello_trait(&person);
    hello_and_goodbye(&person);

    let result = person.say_hello_to("Joko");

    println!("{}", result);

    let result = person.hello();

    println!("{}", result);

    println!("{}", person.good_bye());
    println!("{}", person.good_bye_to("Joko"));

    //conflict method name implement
    CanSayHello::say_hello(&person);
    Person::say_hello(&person, "Joko")

}

struct SimplePerson {
    name: String
}

impl CanSayGoodBye for SimplePerson {
    fn good_bye(&self) -> String {
        format!("Goodbye, my name is {}", self.name)
    }

    fn good_bye_to(&self, name: &str) -> String {
        format!("Goodbye {}, my name is {}", name, self.name)
    }
}

fn create_person(name:String) -> impl CanSayGoodBye{
    SimplePerson { name }
}

#[test]
fn test_return_trait() {
    let person = create_person(String::from("Arif"));
    println!("{}", person.good_bye());
    println!("{}", person.good_bye_to("Joko"));
}

// generic di struct
struct Point<T> {
    x: T,
    y: T,
}

#[test]
fn test_generic_struct() {
    let integer: Point<i32> = Point::<i32> {
        x: 5, y: 10
    };

    let float: Point<f64> = Point::<f64> {
        x: 1.0, y: 3.0
    };

    println!("integer x: {} y: {}", integer.x, integer.y);
    println!("float x: {} y: {}", float.x, float.y);

}

// generic di enum
enum Value<T> {
    NONE,
    VALUE(T),
}

#[test]
fn test_generic_enum() {
    let value: Value<i32> = Value::<i32>::VALUE(10);
    match value{
        Value::NONE => {
            println!("none");
        }
        Value::VALUE(value) => {
            print!("value: {}", value);
        }
    }
}

struct Hi<T: CanSayGoodBye>{
    value: T,
}

#[test]
fn test_generic_struct_with_trait(){
    let hi = Hi::<SimplePerson>{
        value: SimplePerson {
            name: String::from("Arif"),
        },
    };
    println!("{}", hi.value.name);
}

fn min<T: PartialOrd>(value1: T, value2: T) -> T {
    if value1 < value2 {
        value1
    } else {
        value2
    }
}

#[test]
fn generic_in_function() {
    let result = min::<i32>(10,20);
    println!("{}", result);
}

impl<T> Point<T> {
    fn get_x(&self) -> &T {
        &self.x
    }
    fn get_y(&self) -> &T {
        &self.y
    }
}

#[test]
fn test_generic_method() {
    let point: Point<i32> = Point {x:5, y:10};
    println!("{}", point.get_x());
    println!("{}", point.get_y());
    println!("{}", point.get_value());
}

trait GetValue<T> where T: PartialOrd{
    fn get_value(&self) -> &T;
}

impl<T> GetValue<T> for Point<T> where T: PartialOrd{
    fn get_value(&self) -> &T {
        &self.x
    }
}

// default generic type
struct PointX<T = i32> {
    x: T,
    y: T,
}

#[test]
fn test_generic_default_value(){
    let point = PointX{ x: 10, y: 20};
    println!("x: {} y: {}", point.x, point.y);

    let point = Point{ x: 10.5, y: 20.5};
    println!("x: {} y: {}", point.x, point.y);

    let point = Point{ x: 10.5, y: 20.5};
    println!("x: {} y: {}", point.x, point.y);
}

// overloading operator
use core::ops::Add;
struct Apple {
    quantity: i32,
}

impl Add for Apple {
    type Output = Apple;

    fn add(self, rhs: Self) -> Self::Output {
        Apple {
            quantity: self.quantity + rhs.quantity,
        }
    }
}

#[test]
fn test_operator_add() {
    let apple1 = Apple{quantity: 10};
    let apple2 = Apple{quantity: 10};

    let apple3 = apple1 + apple2;
    println!("{}", apple3.quantity);
}

// option enum untuk null atau undefined
fn double(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i * 2),
    }
}

#[test]
fn test_optional_value() {
    let result = double(Some(3));
    println!("{:?}", result);

    let result = double(None);
    println!("{:?}", result);
}

impl PartialEq for Apple {
    fn eq(&self, other: &Self) -> bool {
        self.quantity == other.quantity
    }
}

impl PartialOrd for Apple {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.quantity.partial_cmp(&other.quantity)
    }
}

#[test]
fn test_compare(){
    let apple1 = Apple{quantity: 10};
    let apple2 = Apple{quantity: 20};

    println!("Apple == Apple 2: {}", apple1 == apple2);
    println!("Apple > Apple 2: {}", apple1 > apple2);
    println!("Apple < Apple 2: {}", apple1 < apple2);

}

#[test]
fn test_string_manipulation() {
    let s = String::from("Arif Muhammad");

    println!("{}", s.to_uppercase());
    println!("{}", s.to_lowercase());
    println!("{}", s.len());
    println!("{}", s.replace("Arif", "Joko"));
    println!("{}", s.starts_with("Arif"));
    println!("{}", s.ends_with("Muhammad"));
    println!("{}", s.trim());
    println!("{}", &s[0..5]);
    println!("{:?}", s.get(0..5));
}

// implementasi debug
use std::{cell::{Ref, RefCell}, collections::{BTreeMap, BTreeSet, HashMap, HashSet, LinkedList, VecDeque}, fmt::{Debug, Formatter}, ops::Deref, rc::Rc, result, thread::Result, vec};

struct Category {
    id: String,
    name: String,
}

impl Debug for Category {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Category")
        .field("id", &self.id)
        .field("name", &self.name)
        .finish()
    }
}

#[test]
fn test_format(){
    let category = Category{
        name: String::from("Gadget"),
        id: String::from("GADGET")
    };

    println!("{:?}", category);
}

// penerapan closure
#[test]
fn test_closure() {
    let sum = |value1:i32, value2:i32| -> i32 {
        value1 + value2
    };

    let result = sum(10, 10);
    println!("{}", result);
}

//penerapan closure as parameter
fn print_with_filter(value: String, filter: fn(String) -> String) {
    let result = filter(value);
    println!("{}", result);
}

#[test]
fn test_closure_as_parameter() {
    let filter = |value: String| -> String {
        value.to_uppercase()
    };
    print_with_filter(String::from("Arif"), filter); 
}

//penerapan function as closure
fn to_uppercase(value: String) -> String {
    value.to_uppercase()
}

#[test]
fn test_function_as_closure() {
    print_with_filter(String::from("Arif"), to_uppercase); 
}

struct Counter {
    counter: i32
}

impl Counter {
    fn increment(&mut self){
        self.counter += 1;
        println!("Increment");
    }
}

#[test]
fn test_closure_scope_with_struct(){
    let mut counter = Counter { counter : 0 };
    counter.increment();
    counter.increment();
    counter.increment();

    println!("Counter: {}", counter.counter);
}


// implementasi sequences vector
#[test]
fn test_vector() {
    let mut names: Vec<String> = Vec::<String>::new();
    names.push(String::from("Arif"));
    names.push(String::from("Muhammad"));

    for name in &names {
        println!("{}", name);
    }
}

// implementasi sequences vector deque
#[test]
fn test_vector_deque() {
    let mut names: VecDeque<String> = VecDeque::<String>::new();
    names.push_back(String::from("Arif"));
    names.push_front(String::from("Muhammad"));

    for name in &names {
        println!("{}", name);
    }
}

// implementasi sequences linkedlist, linkedlist tidak bisa diakses dengan index
#[test]
fn test_linked_list() {
    let mut names: LinkedList<String> = LinkedList::<String>::new();
    names.push_back(String::from("Arif"));
    names.push_front(String::from("Muhammad"));

    for name in &names {
        println!("{}", name);
    }
}


// implementasi maps menggunakan HashMap
#[test]
fn test_hash_map() {
    let mut map: HashMap<String, String> = HashMap::new();
    map.insert(String::from("name"), String::from("Arif"));
    map.insert(String::from("age"), String::from("35"));

    let name = map.get("name");
    let age = map.get("age");

    println!("Name: {}", name.unwrap());
    println!("Age: {}", age.unwrap());

    for entry in map{
        println!("{}: {}", entry.0, entry.1);
    }
}

// implementasi maps menggunakan btreemap
#[test]
fn test_btree_map() {
    let mut map: BTreeMap<String, String> = BTreeMap::new();
    map.insert(String::from("name"), String::from("Arif"));
    map.insert(String::from("age"), String::from("35"));
    map.insert(String::from("country"), String::from("Indonesia"));


    for entry in map{
        println!("{}: {}", entry.0, entry.1);
    }
}

// implementasi sets, harus unique, dan tidak bisa diakses menggunakan index hasilnya tidak akan berurut
#[test]
fn test_hash_set() {
    let mut set: HashSet<String> = HashSet::new();
    set.insert(String::from("Arif"));
    set.insert(String::from("Arif"));
    set.insert(String::from("Muhammad"));
    set.insert(String::from("Muhammad"));



    for name in set{
        println!("{}", name);
    }
}

// implementasi sets, harus unique, dan tidak bisa diakses menggunakan index hasilnya akan berurut
#[test]
fn test_btree_set() {
    let mut set: BTreeSet<String> = BTreeSet::new();
    set.insert(String::from("Muhammad"));
    set.insert(String::from("Muhammad"));
    set.insert(String::from("Arif"));
    set.insert(String::from("Arif"));



    for name in set{
        println!("{}", name);
    }
}


// iterator implementation
#[test]
fn test_iterator() {
    let array: [i32;5] = [1,2,3,4,5];
    let mut iterator = array.iter();

    while let Some(value) = iterator.next(){
        println!("{}", value);
    }

    for value in iterator {
        println!("{}", value);
    }
}

// iterator method implementation
#[test]
fn test_iterator_method() {
    let vector: Vec<i32> = vec![1,2,3,4,5,6,7,8,9,10];
    println!("Vector: {:?}", vector);
    let sum: i32 = vector.iter().sum();
    println!("Sum : {}", sum);
    let count: usize = vector.iter().count();
    println!("Count: {}", count);
    let doubled: Vec<i32> = vector.iter().map(|x| x * 2).collect();
    println!("Doubled: {:?}", doubled);
    let odd: Vec<&i32> = vector.iter().filter(|x| *x % 2 != 0).collect();
    println!("Odd: {:?}", odd);
    let even: Vec<&i32> = vector.iter().filter(|x| *x % 2 == 0).collect();
    println!("Even: {:?}", even);
}

// Unrecoverable Error
fn connect_database(host: Option<String>){
    match host {
        None => {
            panic!("No database host provided");
        }
        Some(host) => {
            println!("Connecting to database {}", host);
        }
    }
}

#[test]
fn test_panic() {
    connect_database(Some(String::from("localhost")));
    // connect_database(None);
}

// Recoverable Error
// fn connect_cache(host: Option<String>) -> Result<String, String> {
//     match host {
//         Some(host) => {
//             Ok(host)   
//         }
//         None => {
//             panic!(String::from("No cache host provided"))
//         }
            
//     }
// }

// #[test]
// fn test_recoverable_error() {
//     let host = connect_cache(Some("localhost".to_string()))
//         .expect("Failed to connect to cache");

//     println!("Connected to cache at {}", host);
//     assert!(true);
// }

// fn connect_email(host: Option<String>) -> Result<String, String> {
//     match host {
//         None => {
//             Err(Box::new("No email host provided".to_string()))
//         }
//         Some(host) => {
//             Ok(host)
//         }
//     }
// }

// fn connect_application(host: Option<String>) -> Result<String, String> {
//     connect_cache(host.clone())?;
//     connect_email(host.clone())?;
//     Ok("Connected to application".to_string())
// }

// #[test]
// fn test_application_error() {
//     let result = connect_application(Some("localhost".to_string()));
//     match result {
//         Ok(host) => {println!("success connect to host : {}", host)}
//         Err(err) => {println!("Error with message : {}", err)}
//     }
// }

// test dangling reference
#[test]
fn test_dangling_reference() {
    let r: &i32;
    {
        let x = 5;
        // r = &x;
    }
    r = &40;
    println!("{}", r);
}

// penerapan lifetime annotatioon in function
fn longest<'a>(value1: &'a str, value2: &'a str) -> &'a str{
    if value1.len() > value2.len(){
        value1
    } else {
        value2
    }
}

#[test]
fn test_lifetime_annotation(){
    let value1 = "Arif";
    let value2="Muhammad";
    let result = longest(value1, value2);
    println!("result {}", result);
}

fn test_lifetime_annotation_dangling_reference(){
    let string1 = String::from("Arif");
    let string2 = String::from("Muhammad");
    let result;
    {
        // let string2 = String::from("Muhammad");
        result = longest(string1.as_str(), string2.as_str());

    }
    println!("{}",result);
}

// penerapan lifetime reference di struct
struct Student<'a, 'b> {
    name: &'a str,
    last_name: &'b str
}

// penerapan lifetime di method
impl<'a, 'b> Student<'a, 'b>{
    fn longest_name(&self, student: &Student<'a, 'b>) -> &'a str {
        if self.name.len() > student.name.len() {
            self.name
        } else {
            student.name
        }
    }
}

fn longest_student_name<'a, 'b>(student1: &Student<'a, 'b>, student2: &Student<'a, 'b>) -> &'a str {
    if student1.name.len() > student2.name.len() {
        student1.name
    } else {
        student2.name
    }
}

#[test]
fn test_student(){
    let student = Student{
        name: "Arif",
        last_name: "Muhammad"
    };
    println!("{}", student.name);
    let student2 = Student{
        name: "Jakar",
        last_name: "Widada"
    };
    println!("{}", student2.name);
    let result = longest_student_name(&student, &student2);
    println!("{}", result);

    let result = student.longest_name(&student2);
    println!("{}", result);
}

struct Teacher<'a, ID>
    where 
        ID: Ord,
        {
            id: ID,
            name: &'a str,
        }

#[test]
fn test_lifetime_annotation_generic_struct() {
    let teacher: Teacher<i32> = Teacher { id: 1, name: "Arif" };
    println!("teacher: {} - {}", teacher.id, teacher.name );
}

// Attribute Implement
#[derive(Debug, PartialEq, PartialOrd)]
struct Company {
    name: String,
    location: String,
    website: String,
}

#[test]
fn test_attribute_debug(){
    let company = Company {
        name: "Rust".to_string(),
        location: "USA".to_string(),
        website: "https://www.rust-lang.org".to_string(),
    };

    let company2 = Company {
        name: "Rust".to_string(),
        location: "USA".to_string(),
        website: "https://www.rust-lang.org".to_string(),
    };

    println!("{:?}", company);

    let result = company == company2;
    println!("{}", result);

    let result = company > company2;
    println!("{}", result);
}

#[test]
fn test_box() {
    let value:Box<i32> = Box::new(10);
    println!("{}", value);
    display_number(*value);
    display_number_reference(&value)
}

fn display_number(value: i32){
    println!("{}", value);
}

fn display_number_reference(value: &i32){
    println!("{}", value);
}


// implement Smart Pointer menggunakan Box
#[derive(Debug)]
enum ProductCategory {
    Of(String, Box<ProductCategory>),
    End
}

#[test]
fn test_box_enum() {
    let category = ProductCategory::Of(
        "Laptop".to_string(),
        Box::new(ProductCategory::Of(
            "Dell".to_string(),
            Box::new(ProductCategory::End)
        )),
    );
    print_category(&category)
}

fn print_category(category: &ProductCategory){
    println!("{:?}",category);
}

// implement dereference
#[test]
fn test_dereference() {
    let value1 = Box::new(10);
    let value2 = Box::new(20);

    let result = *value1 * *value2;
    println!("{}", result);

}

// implement dereference Trait
struct MyValue<T> {
    value: T,
}

impl<T> Deref for MyValue<T> {
    type Target = T;
    
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

#[test]
fn test_deref() {
    let value = MyValue { value: 10 };
    let realValue: i32 = *value;
    println!("value: {}", realValue);
}

fn say_hello_reference(name: &String){
    println!("Hello {}", name);
}

#[test]
fn test_deref_reference(){
    let name = MyValue{
        value: "Arif".to_string()
    };
    say_hello_reference(&name);
}

// Drop Implementation
struct Book {
    title: String
}

impl Drop for Book {
    fn drop(&mut self) {
        println!("Dropping Book: {}", self.title)
    }
}

#[test]
fn test_drop(){
    let book = Book{ title: "Rust Programming".to_string()};
    println!("{}", book.title)
}

// implement multiple ownership menggunakan Rc
 enum Brand {
    Of(String, Rc<Brand>),
    End
 }

 #[test]
 fn test_multiple_ownership() {
    let apple: Rc<Brand> = Rc::new(Brand::Of("Apple".to_string(), Rc::new(Brand::End)));
    println!("Apple reference count: {}", Rc::strong_count(&apple));

    let laptop:Brand= Brand::Of("Laptop".to_string(), Rc::clone(&apple));
    println!("Apple reference count: {}", Rc::strong_count(&apple));

    {
        let smartphone:Brand= Brand::Of("Smartphone".to_string(), Rc::clone(&apple));
        println!("Apple reference count: {}", Rc::strong_count(&apple));
    }

    println!("Apple reference count: {}", Rc::strong_count(&apple));
 }

// interior mutability menggunakan RefCell
#[derive(Debug)]
struct Seller {
    name: RefCell<String>,
    active: RefCell<bool>
}

#[test]
fn test_ref_cell() {
    let seller = Seller {
        name: RefCell::new("Arif".to_string()),
        active: RefCell::new(true),
    };

    let mut result = seller.name.borrow_mut();
    *result = "Jaka".to_string();

    println!("{:?}", result);
}

// implement static, bedanya dengan const, Static datanya bisa dirubah2 akan tetapi static tidak. kalau ingin tetap menggunakan gunakan unsafe karena kemungkinana race condition nya besar

static APPLICATION: &str = "My Application";

#[test]
fn test_static() {
    println!("{}", APPLICATION);
}

static mut COUNTER: u32 = 0;

unsafe fn increment(){
    COUNTER +=1;
}

#[test]
fn test_unsafe(){
    unsafe{
        increment();
        COUNTER += 1;
        println!("Counter : {}", COUNTER);
    }

}


// implement declarative macro (metaprogramming)
macro_rules! hi {
    () => {
        println!("Test Macro!");
    };
    ($name: expr) => {
        println!("Hi {}!", $name);
    }
}

#[test]
fn test_macro() {
    hi!();
    hi!("Arif");
    hi!{
        "Arif"
    }

    let name = "Jaka";
    hi!(name);
}

macro_rules! iterate {
    ($array: expr) => {
        for i in $array {
            println!("{}", i);
        }
    };
    ($($item: expr), *) => {
        $(
            println!("{}", $item);
        )*
    };
}

#[test]
fn test_macro_iterate(){
    iterate!([1,2,3,4,5,6,7,8,9,10]);
    iterate!(1,2,3,4,5,6,7,8,9,10);

}


