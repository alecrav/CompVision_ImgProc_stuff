fn main() {

}

struct Foo {num: i32,bar: Bar,}


struct Bar {x: i32,y: bool,}

fn get_foobar_mut(foo: &mut Foo) -> &mut Bar {
    &mut foo.bar
}

fn test2(mut foo: Foo) {
    let bar = get_foobar_mut(&mut foo);
    println!("{}", bar.x);
    println!("{}", foo.num);
}

impl Foo {
    pub fn new(arg: i32) -> Foo {
        assert!(arg > 0);
        Foo {
        num: arg,
        bar: Bar::new(arg - 10),
    }
    }

    pub fn get_bar(&self) -> &Bar {
        &self.bar
    }

    pub fn inc_num(&mut self) {
        if self.num < i32::MAX {
            self.num += 1;
        }
    }
}

impl Bar {
    pub fn new(arg: i32) -> Bar {
        Bar {
            x: arg,
            y: arg > 0,
        }
    }
    
    pub fn get_y_mut(&mut self) -> &mut bool {
        &mut self.y
    }
}