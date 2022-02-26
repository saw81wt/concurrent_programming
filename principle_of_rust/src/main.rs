// 2.3.3 借用

struct Apple {} // リンゴ
struct Gold {} // お金
//struct FullStomach {} // 満腹感

fn get_gold(_: Apple) -> Gold {
    Gold{}
}

//fn get_full_estomach(_: Apple) -> FullStomach {
//    FullStomach{}
//}

fn my_func5() {
    let a = Apple{};

    let _g = get_gold(a);

    // 以下はコンパイルエラー
    // get_full_estomach(a);
}

// 2.3.7 trail
use std::ops::Add;

#[derive(Clone, Copy)]
struct Vec2 {
    x: f64,
    y: f64
}

impl  Add for Vec2 {
    type Output = Vec2;

    fn add(self, rhs: Vec2) -> Vec2 {
        Vec2 {
            x: self.x + rhs.x,
            y: self.y + rhs.y
        }
    }
}

fn add_3times<T> (a: T) -> T
where T: Add<Output = T> + Copy
{
    a + a + a
}

fn my_func10() {
    let v1 = Vec2{x: 10.0, y: 5.0};
    let v2 = Vec2{x: 3.1, y: 8.7};
    let v = v1 + v2;
    println!("v1.x = {}, v1.y = {}", v1.x, v1.y);
    println!("v2.x = {}, v2.y = {}", v2.x, v2.y);
    println!("v.x = {}, v.y = {}", v.x, v.y);

    let v3 = add_3times(v1);
    println!("v3.x = {}, v3.y = {}", v3.x, v3.y);
}

// 2.3.9 thread
use std::thread::spawn;

fn hello() {
    println!("Hello world!");
}

fn my_func11() {
    let _ = spawn(hello).join();

    let h = || println!("Hello World!");
    let _ = spawn(h).join();
}

fn my_func12() {
    let v = 10;
    let f = move || v * 2;

    let result = spawn(f).join();
    println!("result = {:?}", result);

    match spawn(|| panic!("Im panicked!")).join() {
        Ok(_) => {
            println!("successd");
        },
        Err(a) => {
            let s = a.downcast_ref::<&str>();
            println!("failed: {:?}", s);
        }
    }
}

fn main() {
    println!("Hello, world!");
    my_func5();

    my_func10();

    my_func11();

    my_func12();
}
