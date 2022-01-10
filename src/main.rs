#![allow(warnings)]
/* Basic prefix in tailwindcss
        - w
        - h
        - p
        - m
        - max ,min
        - shadow
        - ring
        - outline
        - list
        - border
        - font
        - bg
        - text
        - overflow
        - scroll
        - backdrop
        - animate

        - transition
        - transform
*/

fn main() {
    const INITIAL_CLASS: &str = "
        w-full 
        max-w-5xl 
        h-64 
        text-9xl text-blue-700
        font-thin font-sans
        outline-none
        min-h-screen 
        p-10 m-8 border-2
        ";
    // let mut init_itra = INITIAL_CLASS.chars();
    // loop {
    //     match init_itra.next() {
    //         Some(v) => {
    //             if v.to_string() == "w".to_string() {
    //                 println!("THis mught be width {} {} ", v, init_itra.next().unwrap());
    //             }
    //             print!("{}", v);
    //         }
    //         None => {
    //             break;
    //             println!("Loop breaked")
    //         }
    //     }
    // }

    let mut final_class = String::new();
    /* Sizing */
    let sizing = ["w", "h", "max", "min"];
    /* Box Model */
    let box_model = ["m", "p", "rounded", "border", "divide", "outline", "ring"];
    /* Flexbox-Grid */
    let flexbox = ["basis", "flex", "grow", "shrink"];
    let grid = ["grid", "col", "row", "auto-cols"];
    let alignment = ["place", "self", "items", "content", "justify"];

    /* Typography */
    let fonts = [
        "font",
        "italic",
        "not-italic",
        "antialiased",
        "subpixel-antialiased",
    ];

    let text = [
        "text",
        "underline",
        "overline",
        "line-through",
        "no-underline",
        "decoration",
        "uppercase",
        "lowercase",
        "capitalize",
        "normal-case",
        "truncate",
        "indent",
        "align",
        "whitespace",
        "break-normal",
        "break-words",
        "break-all",
        "tracking",
        "leading",
    ];

    let lists = ["list"];
    let background = ["bg", "from", "via", "to"];

    /* Visuals */
    let shadows = ["shadow"];

    let mut intermediate_class = String::new();
    let mut inter_mediate = String::new();

    /* For Font */
    for class_value in INITIAL_CLASS.split_ascii_whitespace() {
        let (first, _) = class_value.split_once("-").unwrap();
        // println!("Boolean {} {}", sizing.contains(&first), _);
        if fonts.contains(&first) {
            final_class.insert_str(0, class_value);
            final_class.insert_str(0, " ");
        } else {
            intermediate_class.push_str(class_value);
            intermediate_class.push_str(" ");
        }
    }
    final_class.insert_str(0, "\n");
    println!(
        "Intermediate Class {}, \n Final Class {} \n\n",
        intermediate_class, final_class
    );
    /* For Text */
    for class_value in intermediate_class.split_ascii_whitespace() {
        let (first, _) = class_value.split_once("-").unwrap();
        // println!("Boolean {} {}", sizing.contains(&first), _);
        if text.contains(&first) {
            final_class.insert_str(0, class_value);
            final_class.insert_str(0, " ");
        } else {
            inter_mediate.push_str(class_value);
            inter_mediate.push_str(" ");
        }
    }
    final_class.insert_str(0, "\n");
    intermediate_class = inter_mediate;
    inter_mediate = String::new();
    println!(
        "Intermediate Class {}, \n Final Class {} \n\n",
        intermediate_class, final_class
    );
    /* Box Model */
    for class_value in intermediate_class.split_ascii_whitespace() {
        let (first, _) = class_value.split_once("-").unwrap();
        // println!("Boolean {} {}", sizing.contains(&first), _);
        if box_model.contains(&first) {
            final_class.insert_str(0, class_value);
            final_class.insert_str(0, " ");
        } else {
            inter_mediate.push_str(class_value);
            inter_mediate.push_str(" ");
        }
    }
    final_class.insert_str(0, "\n");
    intermediate_class = inter_mediate;
    inter_mediate = String::new();
    println!(
        "Intermediate Class {}, \n Final Class {} \n\n",
        intermediate_class, final_class
    );
    /* For Sizing */
    for class_value in intermediate_class.split_ascii_whitespace() {
        let (first, _) = class_value.split_once("-").unwrap();
        // println!("Boolean {} {}", sizing.contains(&first), _);
        if sizing.contains(&first) {
            final_class.insert_str(0, class_value);
            final_class.insert_str(0, " ");
        } else {
            inter_mediate.push_str(class_value);
            inter_mediate.push_str(" ");
        }
    }
    final_class.insert_str(0, "\n");

    println!("{} ", final_class)
}
