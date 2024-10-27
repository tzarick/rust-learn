#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
    area: Option<u32>,
}

impl Rectangle {
    fn area(&self) -> u32 { // &self is shorthand for self: &Self
        self.width * self.height
    }

    fn set_area(&mut self) {
        self.area = Some(self.area());
    }

    fn has_width(&self) -> bool {
        self.width > 0
    }

    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
            area: None,
        }

    }
}

fn main() {
    let width = 50;
    let height = 80;
    // let rect = (width, height);
    let mut rect = Rectangle { width, height: dbg!(height * 30), area: None };

    rect.set_area();


    println!("has a width? {}", rect.has_width());
    println!(
        "the area of the rectangle is {} square pixels.",
        rect.area.unwrap_or_default()
    );

    println!("rect is {rect:#?}");
    dbg!(&rect); // takes ownership, so pass in a reference. It returns ownership as a return value

    let square = Rectangle::square(600);

    modify_area(rect.area);
    println!("square is {square:#?}");
}

fn modify_area(area: Option<u32>) -> Option<u32> {
    match area {
        Some(area) => Some(area * 2),
        None => None,
    }
}