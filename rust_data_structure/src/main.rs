fn main() {
    // 元组
    let tuple_e = ('e', 5i32, true);
    println!(
        "Is '{}' the {}th letter of the alphabet? {}",
        tuple_e.0, tuple_e.1, tuple_e.2
    );

    // =========结构体 =================
    // 经典结构体
    struct Student {
        name: String,
        level: u8,
        remote: bool,
    }
    // 元组结构体
    struct Grades(char, char, char, char, f32);
    // 单元结构体
    struct Unit;

    // 实列化结构体
    let user_1 = Student {
        name: String::from("Constance Sharma"),
        remote: true,
        level: 2,
    };
    let user_2 = Student {
        name: String::from("Dyson Tan"),
        level: 2,
        remote: true,
    };

    let mark_1 = Grades('A', 'A', 'B', 'A', 3.75);
    let mark_2 = Grades('B', 'A', 'B', 'C', 3.25);
    println!(
        "{}, level {} Remote: {}. Grades: {}, {}, {}, {}. Average: {}",
        user_1.name, user_1.level, user_1.remote, mark_1.0, mark_1.1, mark_1.2, mark_1.3, mark_1.4
    );
    println!(
        "{}, level {} Remote: {}. Grades: {}, {}, {}, {}. Average: {}",
        user_2.name, user_2.level, user_2.remote, mark_2.0, mark_2.1, mark_2.2, mark_2.3, mark_2.4
    );

    // 枚举
    /* enum WebEvent { */
    /* WELoad, */
    /* WEKwys(String, char), */
    /* WEClick { x: i64, y: i64 }, */
    /* } */
    #[derive(Debug)]
    struct KeyPress(String, char);

    #[derive(Debug)]
    struct MouseClick {
        x: i64,
        y: i64,
    }

    #[derive(Debug)]
    enum WebEvent {
        WELoad(bool),
        WEClick(MouseClick),
        WEKwys(KeyPress),
    }
    let we_load = WebEvent::WELoad(true);
    let click = MouseClick { x: 100, y: 250 };
    println!("Mouse click location: {}, {}", click.x, click.y);

    let we_click = WebEvent::WEClick(click);
    let keys = KeyPress(String::from("Ctrl+"), 'N');

    println!("\nKeys pressed: {}{}", keys.0, keys.1);
    let we_key = WebEvent::WEKwys(keys);
    println!(
        "\nWebEvent enum structure: \n\n {:#?} \n\n {:#?} \n\n {:#?}",
        we_load, we_click, we_key
    );
}
