// use std::time::Instant;
use std::cmp::Ordering;

pub fn sort_tailwindcss_classes(initial_class: &str) -> String {
    let mut class_iterator = initial_class.chars();

    let mut box_model: Vec<(String, u64)> = Vec::new();

    let mut positioning: Vec<(String, u64)> = Vec::new();

    let mut alignment: Vec<(String, u64)> = Vec::new();

    let mut flex_grid: Vec<(String, u64)> = Vec::new();

    let mut typography: Vec<(String, u64)> = Vec::new();
    // Visuals :- Classes like bg, shadows, backdrop-shadows
    let mut visuals: Vec<(String, u64)> = Vec::new();
    // Effects brightness blur filters as well mix
    let mut effects: Vec<(String, u64)> = Vec::new();
    // Transform
    let mut transforms: Vec<(String, u64)> = Vec::new();
    // ANimation & Transition
    let mut motion: Vec<(String, u64)> = Vec::new();
    // Misc will contain classes like caret color or scroll snap classes;
    let mut misc: Vec<(String, u64)> = Vec::new();

    // let start = Instant::now();

    loop {
        match class_iterator.next() {
            Some(i) => {
                let mut word = String::from(i);
                let mut variant = String::new();
                // let time_start = Instant::now();
                'word_loop: loop {
                    match class_iterator.next() {
                        Some(t) => match t {
                            ' ' => {
                                break 'word_loop;
                            }
                            ':' => {
                                variant = variant + &word;
                                variant.push(t);
								word = String::new();
                            }
                            '\t' => {
                                break 'word_loop;
                            }
                            '\n' => {
                                break 'word_loop;
                            }
                            _ => {
                                word.push(t);
                            }
                        },
                        None => break,
                    }
                }
                // let time_end = time_start.elapsed();
                // println!(" Dur for Indentifying the word {:?} ", time_end);
                let mut init_itra = word.chars();
                match init_itra.next() {
                    Some(v) => {
						match v {
                            'a' => {
                                let d1 = init_itra.next().unwrap();
                                match d1 {
                                    'b' => {
                                        //'absolute'
                                        let mut v2 = init_itra.next().unwrap();
                                        let mut v3 = String::new();
                                        loop {
                                            v3 = v3 + &String::from(v2);
                                            match init_itra.next() {
                                                Some(next_char_value) => v2 = next_char_value,
                                                None => break,
                                            }
                                        }
                                        if variant == "" {
                                            let v4 = v.to_string() + &d1.to_string() + &v3;
                                            positioning.push((v4, 1));
                                        } else {
                                            let v4 = variant.to_string()
                                                + &v.to_string()
                                                + &d1.to_string()
                                                + &v3;
                                            positioning.push((v4, 1));
                                        }
                                    }
                                    's' => {
                                        //'aspect'
                                        let mut v2 = init_itra.next().unwrap();
                                        let mut v3 = String::new();
                                        loop {
                                            v3 = v3 + &String::from(v2);
                                            match init_itra.next() {
                                                Some(next_char_value) => v2 = next_char_value,
                                                None => break,
                                            }
                                        }
                                        if variant == "" {
                                            let v4 = v.to_string() + &d1.to_string() + &v3;
                                            box_model.push((v4, 0));
                                        } else {
                                            let v4 = variant.to_string()
                                                + &v.to_string()
                                                + &d1.to_string()
                                                + &v3;
                                            box_model.push((v4, 0));
                                        }
                                    }
                                    'u' => {
                                        let d2 = init_itra.next().unwrap();
                                        match d2 {
                                            't' => {
                                                let d3 = init_itra.next().unwrap();
                                                match d3 {
                                                    'o' => {
                                                        let d4 = init_itra.next().unwrap();
                                                        match d4 {
                                                            '-' => {
                                                                let d5 = init_itra.next().unwrap();
                                                                match d5 {
                                                                    'c' => {
                                                                        //'auto-cols'
                                                                        let mut v2 = init_itra
                                                                            .next()
                                                                            .unwrap();
                                                                        let mut v3 = String::new();
                                                                        loop {
                                                                            v3 = v3
                                                                                + &String::from(v2);
                                                                            match init_itra.next() {
                                                                                Some(
                                                                                    next_char_value,
                                                                                ) => v2 =
                                                                                    next_char_value,
                                                                                None => break,
                                                                            }
                                                                        }
                                                                        if variant == "" {
                                                                            let v4 = v.to_string()
                                                                                + &d1.to_string()
                                                                                + &d2.to_string()
                                                                                + &d3.to_string()
                                                                                + &d4.to_string()
                                                                                + &d5.to_string()
                                                                                + &v3;
                                                                            flex_grid.push((v4, 6));
                                                                        } else {
                                                                            let v4 = variant
                                                                                .to_string()
                                                                                + &v.to_string()
                                                                                + &d1.to_string()
                                                                                + &d2.to_string()
                                                                                + &d3.to_string()
                                                                                + &d4.to_string()
                                                                                + &d5.to_string()
                                                                                + &v3;
                                                                            flex_grid.push((v4, 6));
                                                                        }
                                                                    }
                                                                    'r' => {
                                                                        //'auto-rows'
                                                                        let mut v2 = init_itra
                                                                            .next()
                                                                            .unwrap();
                                                                        let mut v3 = String::new();
                                                                        loop {
                                                                            v3 = v3
                                                                                + &String::from(v2);
                                                                            match init_itra.next() {
                                                                                Some(
                                                                                    next_char_value,
                                                                                ) => v2 =
                                                                                    next_char_value,
                                                                                None => break,
                                                                            }
                                                                        }
                                                                        if variant == "" {
                                                                            let v4 = v.to_string()
                                                                                + &d1.to_string()
                                                                                + &d2.to_string()
                                                                                + &d3.to_string()
                                                                                + &d4.to_string()
                                                                                + &d5.to_string()
                                                                                + &v3;
                                                                            flex_grid.push((v4, 7));
                                                                        } else {
                                                                            let v4 = variant
                                                                                .to_string()
                                                                                + &v.to_string()
                                                                                + &d1.to_string()
                                                                                + &d2.to_string()
                                                                                + &d3.to_string()
                                                                                + &d4.to_string()
                                                                                + &d5.to_string()
                                                                                + &v3;
                                                                            flex_grid.push((v4, 7));
                                                                        }
                                                                    }
                                                                    _ => (),
                                                                }
                                                            }
                                                            _ => (),
                                                        }
                                                    }
                                                    _ => (),
                                                }
                                            }
                                            _ => (),
                                        }
                                    }
                                    'n' => {
                                        let d2 = init_itra.next().unwrap();
                                        match d2 {
                                            't' => {
                                                //'antialiased'
                                                let mut v2 = init_itra.next().unwrap();
                                                let mut v3 = String::new();
                                                loop {
                                                    v3 = v3 + &String::from(v2);
                                                    match init_itra.next() {
                                                        Some(next_char_value) => {
                                                            v2 = next_char_value
                                                        }
                                                        None => break,
                                                    }
                                                }
                                                if variant == "" {
                                                    let v4 = v.to_string()
                                                        + &d1.to_string()
                                                        + &d2.to_string()
                                                        + &v3;
                                                    typography.push((v4, 4));
                                                } else {
                                                    let v4 = variant.to_string()
                                                        + &v.to_string()
                                                        + &d1.to_string()
                                                        + &d2.to_string()
                                                        + &v3;
                                                    typography.push((v4, 4));
                                                }
                                            }
                                            'i' => {
                                                //'animate'
                                                let mut v2 = init_itra.next().unwrap();
                                                let mut v3 = String::new();
                                                loop {
                                                    v3 = v3 + &String::from(v2);
                                                    match init_itra.next() {
                                                        Some(next_char_value) => {
                                                            v2 = next_char_value
                                                        }
                                                        None => break,
                                                    }
                                                }
                                                if variant == "" {
                                                    let v4 = v.to_string()
                                                        + &d1.to_string()
                                                        + &d2.to_string()
                                                        + &v3;
                                                    motion.push((v4, 1));
                                                } else {
                                                    let v4 = variant.to_string()
                                                        + &v.to_string()
                                                        + &d1.to_string()
                                                        + &d2.to_string()
                                                        + &v3;
                                                    motion.push((v4, 1));
                                                }
                                            }
                                            _ => (),
                                        }
                                    }
                                    'l' => {
                                        //'align'
                                        let mut v2 = init_itra.next().unwrap();
                                        let mut v3 = String::new();
                                        loop {
                                            v3 = v3 + &String::from(v2);
                                            match init_itra.next() {
                                                Some(next_char_value) => v2 = next_char_value,
                                                None => break,
                                            }
                                        }
                                        if variant == "" {
                                            let v4 = v.to_string() + &d1.to_string() + &v3;
                                            typography.push((v4, 14));
                                        } else {
                                            let v4 = variant.to_string()
                                                + &v.to_string()
                                                + &d1.to_string()
                                                + &v3;
                                            typography.push((v4, 14));
                                        }
                                    }
                                    'c' => {
                                        //'accent'
                                        let mut v2 = init_itra.next().unwrap();
                                        let mut v3 = String::new();
                                        loop {
                                            v3 = v3 + &String::from(v2);
                                            match init_itra.next() {
                                                Some(next_char_value) => v2 = next_char_value,
                                                None => break,
                                            }
                                        }
                                        if variant == "" {
                                            let v4 = v.to_string() + &d1.to_string() + &v3;
                                            misc.push((v4, 2));
                                        } else {
                                            let v4 = variant.to_string()
                                                + &v.to_string()
                                                + &d1.to_string()
                                                + &v3;
                                            misc.push((v4, 2));
                                        }
                                    }
                                    'p' => {
                                        //'appearance'
                                        let mut v2 = init_itra.next().unwrap();
                                        let mut v3 = String::new();
                                        loop {
                                            v3 = v3 + &String::from(v2);
                                            match init_itra.next() {
                                                Some(next_char_value) => v2 = next_char_value,
                                                None => break,
                                            }
                                        }
                                        if variant == "" {
                                            let v4 = v.to_string() + &d1.to_string() + &v3;
                                            misc.push((v4, 2));
                                        } else {
                                            let v4 = variant.to_string()
                                                + &v.to_string()
                                                + &d1.to_string()
                                                + &v3;
                                            misc.push((v4, 2));
                                        }
                                    }
                                    _ => (),
                                }
                            }
                            'b' => {
                                let d1 = init_itra.next().unwrap();
                                match d1 {
                                    'o' => {
                                        let d2 = init_itra.next().unwrap();
                                        match d2 {
                                            't' => {
                                                //'bottom'
                                                let mut v2 = init_itra.next().unwrap();
                                                let mut v3 = String::new();
                                                loop {
                                                    v3 = v3 + &String::from(v2);
                                                    match init_itra.next() {
                                                        Some(next_char_value) => {
                                                            v2 = next_char_value
                                                        }
                                                        None => break,
                                                    }
                                                }
                                                if variant == "" {
                                                    let v4 = v.to_string()
                                                        + &d1.to_string()
                                                        + &d2.to_string()
                                                        + &v3;
                                                    positioning.push((v4, 5));
                                                } else {
                                                    let v4 = variant.to_string()
                                                        + &v.to_string()
                                                        + &d1.to_string()
                                                        + &d2.to_string()
                                                        + &v3;
                                                    positioning.push((v4, 5));
                                                }
                                            }
                                            'r' => {
                                                let d3 = init_itra.next().unwrap();
                                                match d3 {
                                                    'd' => {
                                                        let d4 = init_itra.next().unwrap();
                                                        match d4 {
                                                            'e' => {
                                                                let d5 = init_itra.next().unwrap();
                                                                match d5 {
                                                                    'r' => {
                                                                        let d6 = init_itra
                                                                            .next()
                                                                            .unwrap();
                                                                        match d6 {
                                                                            '-' => {
                                                                                let d7 = init_itra
                                                                                    .next()
                                                                                    .unwrap();
                                                                                match d7 {
                                                                                    'x' => {
                                                                                        //'border-x'
                                                                                        let mut v2 = init_itra.next().unwrap();
                                                                                        let mut v3 = String::new();
                                                                                        loop {
                                                                                            v3 = v3 + &String::from(v2);
                                                                                            match init_itra.next() {
    																			        Some(next_char_value) => v2 = next_char_value,
    																			        None => break,
    																			    }
                                                                                        }
                                                                                        if variant
                                                                                            == ""
                                                                                        {
                                                                                            let v4 = v.to_string() + &d1.to_string()+ &d2.to_string()+ &d3.to_string()+ &d4.to_string()+ &d5.to_string()+ &d6.to_string()+ &d7.to_string() + &v3;
                                                                                            box_model.push((
																						v4,
    																			  		33,
    																				));
                                                                                        } else {
                                                                                            let v4 = variant.to_string() + &v.to_string() + &d1.to_string()+ &d2.to_string()+ &d3.to_string()+ &d4.to_string()+ &d5.to_string()+ &d6.to_string()+ &d7.to_string() + &v3;
                                                                                            box_model.push((
    																			  v4,
    																			  33,
    																			));
                                                                                        }
                                                                                    }
                                                                                    'y' => {
                                                                                        //'border-y'
                                                                                        let mut v2 = init_itra.next().unwrap();
                                                                                        let mut v3 = String::new();
                                                                                        loop {
                                                                                            v3 = v3 + &String::from(v2);
                                                                                            match init_itra.next() {
    																			        Some(next_char_value) => v2 = next_char_value,
    																			        None => break,
    																			    }
                                                                                        }
                                                                                        if variant
                                                                                            == ""
                                                                                        {
                                                                                            let v4 = v.to_string() + &d1.to_string()+ &d2.to_string()+ &d3.to_string()+ &d4.to_string()+ &d5.to_string()+ &d6.to_string()+ &d7.to_string() + &v3;
                                                                                            box_model.push((
																						v4,
    																			  		34,
    																				));
                                                                                        } else {
                                                                                            let v4 = variant.to_string() + &v.to_string() + &d1.to_string()+ &d2.to_string()+ &d3.to_string()+ &d4.to_string()+ &d5.to_string()+ &d6.to_string()+ &d7.to_string() + &v3;
                                                                                            box_model.push((
    																			  v4,
    																			  34,
    																			));
                                                                                        }
                                                                                    }
                                                                                    't' => {
                                                                                        //'border-t'
                                                                                        let mut v2 = init_itra.next().unwrap();
                                                                                        let mut v3 = String::new();
                                                                                        loop {
                                                                                            v3 = v3 + &String::from(v2);
                                                                                            match init_itra.next() {
    																			        Some(next_char_value) => v2 = next_char_value,
    																			        None => break,
    																			    }
                                                                                        }
                                                                                        if variant
                                                                                            == ""
                                                                                        {
                                                                                            let v4 = v.to_string() + &d1.to_string()+ &d2.to_string()+ &d3.to_string()+ &d4.to_string()+ &d5.to_string()+ &d6.to_string()+ &d7.to_string() + &v3;
                                                                                            box_model.push((
																						v4,
    																			  		35,
    																				));
                                                                                        } else {
                                                                                            let v4 = variant.to_string() + &v.to_string() + &d1.to_string()+ &d2.to_string()+ &d3.to_string()+ &d4.to_string()+ &d5.to_string()+ &d6.to_string()+ &d7.to_string() + &v3;
                                                                                            box_model.push((
    																			  v4,
    																			  35,
    																			));
                                                                                        }
                                                                                    }
                                                                                    'l' => {
                                                                                        //'border-l'
                                                                                        let mut v2 = init_itra.next().unwrap();
                                                                                        let mut v3 = String::new();
                                                                                        loop {
                                                                                            v3 = v3 + &String::from(v2);
                                                                                            match init_itra.next() {
    																			        Some(next_char_value) => v2 = next_char_value,
    																			        None => break,
    																			    }
                                                                                        }
                                                                                        if variant
                                                                                            == ""
                                                                                        {
                                                                                            let v4 = v.to_string() + &d1.to_string()+ &d2.to_string()+ &d3.to_string()+ &d4.to_string()+ &d5.to_string()+ &d6.to_string()+ &d7.to_string() + &v3;
                                                                                            box_model.push((
																						v4,
    																			  		36,
    																				));
                                                                                        } else {
                                                                                            let v4 = variant.to_string() + &v.to_string() + &d1.to_string()+ &d2.to_string()+ &d3.to_string()+ &d4.to_string()+ &d5.to_string()+ &d6.to_string()+ &d7.to_string() + &v3;
                                                                                            box_model.push((
    																			  v4,
    																			  36,
    																			));
                                                                                        }
                                                                                    }
                                                                                    'r' => {
                                                                                        //'border-r'
                                                                                        let mut v2 = init_itra.next().unwrap();
                                                                                        let mut v3 = String::new();
                                                                                        loop {
                                                                                            v3 = v3 + &String::from(v2);
                                                                                            match init_itra.next() {
    																			        Some(next_char_value) => v2 = next_char_value,
    																			        None => break,
    																			    }
                                                                                        }
                                                                                        if variant
                                                                                            == ""
                                                                                        {
                                                                                            let v4 = v.to_string() + &d1.to_string()+ &d2.to_string()+ &d3.to_string()+ &d4.to_string()+ &d5.to_string()+ &d6.to_string()+ &d7.to_string() + &v3;
                                                                                            box_model.push((
																						v4,
    																			  		37,
    																				));
                                                                                        } else {
                                                                                            let v4 = variant.to_string() + &v.to_string() + &d1.to_string()+ &d2.to_string()+ &d3.to_string()+ &d4.to_string()+ &d5.to_string()+ &d6.to_string()+ &d7.to_string() + &v3;
                                                                                            box_model.push((
    																			  v4,
    																			  37,
    																			));
                                                                                        }
                                                                                    }
                                                                                    'b' => {
                                                                                        //'border-b'
                                                                                        let mut v2 = init_itra.next().unwrap();
                                                                                        let mut v3 = String::new();
                                                                                        loop {
                                                                                            v3 = v3 + &String::from(v2);
                                                                                            match init_itra.next() {
    																			        Some(next_char_value) => v2 = next_char_value,
    																			        None => break,
    																			    }
                                                                                        }
                                                                                        if variant
                                                                                            == ""
                                                                                        {
                                                                                            let v4 = v.to_string() + &d1.to_string()+ &d2.to_string()+ &d3.to_string()+ &d4.to_string()+ &d5.to_string()+ &d6.to_string()+ &d7.to_string() + &v3;
                                                                                            box_model.push((
																						v4,
    																			  		38,
    																				));
                                                                                        } else {
                                                                                            let v4 = variant.to_string() + &v.to_string() + &d1.to_string()+ &d2.to_string()+ &d3.to_string()+ &d4.to_string()+ &d5.to_string()+ &d6.to_string()+ &d7.to_string() + &v3;
                                                                                            box_model.push((
    																			  v4,
    																			  38,
    																			));
                                                                                        }
                                                                                    }
                                                                                    _ => {
                                                                                        //'border-'
                                                                                        let mut v2 = init_itra.next().unwrap();
                                                                                        let mut v3 = String::new();
                                                                                        loop {
                                                                                            v3 = v3 + &String::from(v2);
                                                                                            match init_itra.next() {
                                                                                        Some(next_char_value) => v2 = next_char_value,
                                                                                        None => break,
                                                                                    }
                                                                                        }
                                                                                        if variant
                                                                                            == ""
                                                                                        {
                                                                                            let v4 = v.to_string() + &d1.to_string()+ &d2.to_string()+ &d3.to_string()+ &d4.to_string()+ &d5.to_string()+ &d6.to_string()+ &d7.to_string() + &v3;
                                                                                            box_model.push((
																						v4,
                                                                                  		31,
                                                                                	));
                                                                                        } else {
                                                                                            let v4 = variant.to_string() + &v.to_string() + &d1.to_string()+ &d2.to_string()+ &d3.to_string()+ &d4.to_string()+ &d5.to_string()+ &d6.to_string()+ &d7.to_string() + &v3;
                                                                                            box_model.push((
                                                                                  v4,
                                                                                  31,
                                                                                ));
                                                                                        }
                                                                                    }
                                                                                }
                                                                            }
                                                                            _ => (),
                                                                        }
                                                                    }
                                                                    _ => (),
                                                                }
                                                            }
                                                            _ => (),
                                                        }
                                                    }
                                                    _ => (),
                                                }
                                            }
                                            'x' => {
                                                let d3 = init_itra.next().unwrap();
                                                match d3 {
                                                    '-' => {
                                                        let d4 = init_itra.next().unwrap();
                                                        match d4 {
                                                            'd' => {
                                                                //'box-decoration'
                                                                let mut v2 =
                                                                    init_itra.next().unwrap();
                                                                let mut v3 = String::new();
                                                                loop {
                                                                    v3 = v3 + &String::from(v2);
                                                                    match init_itra.next() {
                                                                        Some(next_char_value) => {
                                                                            v2 = next_char_value
                                                                        }
                                                                        None => break,
                                                                    }
                                                                }
                                                                if variant == "" {
                                                                    let v4 = v.to_string()
                                                                        + &d1.to_string()
                                                                        + &d2.to_string()
                                                                        + &d3.to_string()
                                                                        + &d4.to_string()
                                                                        + &v3;
                                                                    box_model.push((v4, 74));
                                                                } else {
                                                                    let v4 = variant.to_string()
                                                                        + &v.to_string()
                                                                        + &d1.to_string()
                                                                        + &d2.to_string()
                                                                        + &d3.to_string()
                                                                        + &d4.to_string()
                                                                        + &v3;
                                                                    box_model.push((v4, 74));
                                                                }
                                                            }
                                                            'b' => {
                                                                //'box-border'
                                                                let mut v2 =
                                                                    init_itra.next().unwrap();
                                                                let mut v3 = String::new();
                                                                loop {
                                                                    v3 = v3 + &String::from(v2);
                                                                    match init_itra.next() {
                                                                        Some(next_char_value) => {
                                                                            v2 = next_char_value
                                                                        }
                                                                        None => break,
                                                                    }
                                                                }
                                                                if variant == "" {
                                                                    let v4 = v.to_string()
                                                                        + &d1.to_string()
                                                                        + &d2.to_string()
                                                                        + &d3.to_string()
                                                                        + &d4.to_string()
                                                                        + &v3;
                                                                    box_model.push((v4, 75));
                                                                } else {
                                                                    let v4 = variant.to_string()
                                                                        + &v.to_string()
                                                                        + &d1.to_string()
                                                                        + &d2.to_string()
                                                                        + &d3.to_string()
                                                                        + &d4.to_string()
                                                                        + &v3;
                                                                    box_model.push((v4, 75));
                                                                }
                                                            }
                                                            'c' => {
                                                                //'box-content'
                                                                let mut v2 =
                                                                    init_itra.next().unwrap();
                                                                let mut v3 = String::new();
                                                                loop {
                                                                    v3 = v3 + &String::from(v2);
                                                                    match init_itra.next() {
                                                                        Some(next_char_value) => {
                                                                            v2 = next_char_value
                                                                        }
                                                                        None => break,
                                                                    }
                                                                }
                                                                if variant == "" {
                                                                    let v4 = v.to_string()
                                                                        + &d1.to_string()
                                                                        + &d2.to_string()
                                                                        + &d3.to_string()
                                                                        + &d4.to_string()
                                                                        + &v3;
                                                                    box_model.push((v4, 76));
                                                                } else {
                                                                    let v4 = variant.to_string()
                                                                        + &v.to_string()
                                                                        + &d1.to_string()
                                                                        + &d2.to_string()
                                                                        + &d3.to_string()
                                                                        + &d4.to_string()
                                                                        + &v3;
                                                                    box_model.push((v4, 76));
                                                                }
                                                            }
                                                            _ => (),
                                                        }
                                                    }
                                                    _ => (),
                                                }
                                            }
                                            _ => (),
                                        }
                                    }
                                    'l' => {
                                        let d2 = init_itra.next().unwrap();
                                        match d2 {
                                            'o' => {
                                                //'block'
                                                let mut v2 = init_itra.next().unwrap();
                                                let mut v3 = String::new();
                                                loop {
                                                    v3 = v3 + &String::from(v2);
                                                    match init_itra.next() {
                                                        Some(next_char_value) => {
                                                            v2 = next_char_value
                                                        }
                                                        None => break,
                                                    }
                                                }
                                                if variant == "" {
                                                    let v4 = v.to_string()
                                                        + &d1.to_string()
                                                        + &d2.to_string()
                                                        + &v3;
                                                    box_model.push((v4, 1));
                                                } else {
                                                    let v4 = variant.to_string()
                                                        + &v.to_string()
                                                        + &d1.to_string()
                                                        + &d2.to_string()
                                                        + &v3;
                                                    box_model.push((v4, 1));
                                                }
                                            }
                                            'u' => {
                                                //'blur'
                                                let mut v2 = init_itra.next().unwrap();
                                                let mut v3 = String::new();
                                                loop {
                                                    v3 = v3 + &String::from(v2);
                                                    match init_itra.next() {
                                                        Some(next_char_value) => {
                                                            v2 = next_char_value
                                                        }
                                                        None => break,
                                                    }
                                                }
                                                if variant == "" {
                                                    let v4 = v.to_string()
                                                        + &d1.to_string()
                                                        + &d2.to_string()
                                                        + &v3;
                                                    effects.push((v4, 3));
                                                } else {
                                                    let v4 = variant.to_string()
                                                        + &v.to_string()
                                                        + &d1.to_string()
                                                        + &d2.to_string()
                                                        + &v3;
                                                    effects.push((v4, 3));
                                                }
                                            }
                                            _ => (),
                                        }
                                    }
                                    'r' => {
                                        let d2 = init_itra.next().unwrap();
                                        match d2 {
                                            'e' => {
                                                let d3 = init_itra.next().unwrap();
                                                match d3 {
                                                    'a' => {
                                                        let d4 = init_itra.next().unwrap();
                                                        match d4 {
                                                            'k' => {
                                                                let d5 = init_itra.next().unwrap();
                                                                match d5 {
                                                                    '-' => {
                                                                        let d6 = init_itra
                                                                            .next()
                                                                            .unwrap();
                                                                        match d6 {
                                                                            'a' => {
                                                                                let d7 = init_itra
                                                                                    .next()
                                                                                    .unwrap();
                                                                                match d7 {
                                                                                    'f' => {
                                                                                        //'break-after'
                                                                                        let mut v2 = init_itra.next().unwrap();
                                                                                        let mut v3 = String::new();
                                                                                        loop {
                                                                                            v3 = v3 + &String::from(v2);
                                                                                            match init_itra.next() {
    																			        Some(next_char_value) => v2 = next_char_value,
    																			        None => break,
    																			    }
                                                                                        }
                                                                                        if variant
                                                                                            == ""
                                                                                        {
                                                                                            let v4 = v.to_string() + &d1.to_string()+ &d2.to_string()+ &d3.to_string()+ &d4.to_string()+ &d5.to_string()+ &d6.to_string()+ &d7.to_string() + &v3;
                                                                                            box_model.push((
																						v4,
    																			  		71,
    																				));
                                                                                        } else {
                                                                                            let v4 = variant.to_string() + &v.to_string() + &d1.to_string()+ &d2.to_string()+ &d3.to_string()+ &d4.to_string()+ &d5.to_string()+ &d6.to_string()+ &d7.to_string() + &v3;
                                                                                            box_model.push((
    																			  v4,
    																			  71,
    																			));
                                                                                        }
                                                                                    }
                                                                                    'l' => {
                                                                                        //'break-all'
                                                                                        let mut v2 = init_itra.next().unwrap();
                                                                                        let mut v3 = String::new();
                                                                                        loop {
                                                                                            v3 = v3 + &String::from(v2);
                                                                                            match init_itra.next() {
    																			        Some(next_char_value) => v2 = next_char_value,
    																			        None => break,
    																			    }
                                                                                        }
                                                                                        if variant
                                                                                            == ""
                                                                                        {
                                                                                            let v4 = v.to_string() + &d1.to_string()+ &d2.to_string()+ &d3.to_string()+ &d4.to_string()+ &d5.to_string()+ &d6.to_string()+ &d7.to_string() + &v3;
                                                                                            typography.push((
																						v4,
    																			  		16,
    																				));
                                                                                        } else {
                                                                                            let v4 = variant.to_string() + &v.to_string() + &d1.to_string()+ &d2.to_string()+ &d3.to_string()+ &d4.to_string()+ &d5.to_string()+ &d6.to_string()+ &d7.to_string() + &v3;
                                                                                            typography.push((
    																			  v4,
    																			  16,
    																			));
                                                                                        }
                                                                                    }
                                                                                    _ => (),
                                                                                }
                                                                            }
                                                                            'b' => {
                                                                                //'break-before'
                                                                                let mut v2 =
                                                                                    init_itra
                                                                                        .next()
                                                                                        .unwrap();
                                                                                let mut v3 =
                                                                                    String::new();
                                                                                loop {
                                                                                    v3 = v3 + &String::from(v2);
                                                                                    match init_itra.next() {
    																	        Some(next_char_value) => v2 = next_char_value,
    																	        None => break,
    																	    }
                                                                                }
                                                                                if variant == "" {
                                                                                    let v4 = v.to_string() + &d1.to_string()+ &d2.to_string()+ &d3.to_string()+ &d4.to_string()+ &d5.to_string()+ &d6.to_string() + &v3;
                                                                                    box_model.push(
                                                                                        (v4, 72),
                                                                                    );
                                                                                } else {
                                                                                    let v4 = variant.to_string() + &v.to_string() + &d1.to_string()+ &d2.to_string()+ &d3.to_string()+ &d4.to_string()+ &d5.to_string()+ &d6.to_string() + &v3;
                                                                                    box_model.push(
                                                                                        (v4, 72),
                                                                                    );
                                                                                }
                                                                            }
                                                                            'i' => {
                                                                                //'break-inside'
                                                                                let mut v2 =
                                                                                    init_itra
                                                                                        .next()
                                                                                        .unwrap();
                                                                                let mut v3 =
                                                                                    String::new();
                                                                                loop {
                                                                                    v3 = v3 + &String::from(v2);
                                                                                    match init_itra.next() {
    																	        Some(next_char_value) => v2 = next_char_value,
    																	        None => break,
    																	    }
                                                                                }
                                                                                if variant == "" {
                                                                                    let v4 = v.to_string() + &d1.to_string()+ &d2.to_string()+ &d3.to_string()+ &d4.to_string()+ &d5.to_string()+ &d6.to_string() + &v3;
                                                                                    box_model.push(
                                                                                        (v4, 73),
                                                                                    );
                                                                                } else {
                                                                                    let v4 = variant.to_string() + &v.to_string() + &d1.to_string()+ &d2.to_string()+ &d3.to_string()+ &d4.to_string()+ &d5.to_string()+ &d6.to_string() + &v3;
                                                                                    box_model.push(
                                                                                        (v4, 73),
                                                                                    );
                                                                                }
                                                                            }
                                                                            'n' => {
                                                                                //'break-normal'
                                                                                let mut v2 =
                                                                                    init_itra
                                                                                        .next()
                                                                                        .unwrap();
                                                                                let mut v3 =
                                                                                    String::new();
                                                                                loop {
                                                                                    v3 = v3 + &String::from(v2);
                                                                                    match init_itra.next() {
    																	        Some(next_char_value) => v2 = next_char_value,
    																	        None => break,
    																	    }
                                                                                }
                                                                                if variant == "" {
                                                                                    let v4 = v.to_string() + &d1.to_string()+ &d2.to_string()+ &d3.to_string()+ &d4.to_string()+ &d5.to_string()+ &d6.to_string() + &v3;
                                                                                    typography
                                                                                        .push((
                                                                                            v4, 16,
                                                                                        ));
                                                                                } else {
                                                                                    let v4 = variant.to_string() + &v.to_string() + &d1.to_string()+ &d2.to_string()+ &d3.to_string()+ &d4.to_string()+ &d5.to_string()+ &d6.to_string() + &v3;
                                                                                    typography
                                                                                        .push((
                                                                                            v4, 16,
                                                                                        ));
                                                                                }
                                                                            }
                                                                            'w' => {
                                                                                //'break-words'
                                                                                let mut v2 =
                                                                                    init_itra
                                                                                        .next()
                                                                                        .unwrap();
                                                                                let mut v3 =
                                                                                    String::new();
                                                                                loop {
                                                                                    v3 = v3 + &String::from(v2);
                                                                                    match init_itra.next() {
    																	        Some(next_char_value) => v2 = next_char_value,
    																	        None => break,
    																	    }
                                                                                }
                                                                                if variant == "" {
                                                                                    let v4 = v.to_string() + &d1.to_string()+ &d2.to_string()+ &d3.to_string()+ &d4.to_string()+ &d5.to_string()+ &d6.to_string() + &v3;
                                                                                    typography
                                                                                        .push((
                                                                                            v4, 16,
                                                                                        ));
                                                                                } else {
                                                                                    let v4 = variant.to_string() + &v.to_string() + &d1.to_string()+ &d2.to_string()+ &d3.to_string()+ &d4.to_string()+ &d5.to_string()+ &d6.to_string() + &v3;
                                                                                    typography
                                                                                        .push((
                                                                                            v4, 16,
                                                                                        ));
                                                                                }
                                                                            }
                                                                            _ => (),
                                                                        }
                                                                    }
                                                                    _ => (),
                                                                }
                                                            }
                                                            _ => (),
                                                        }
                                                    }
                                                    _ => (),
                                                }
                                            }
                                            'i' => {
                                                //'brightness'
                                                let mut v2 = init_itra.next().unwrap();
                                                let mut v3 = String::new();
                                                loop {
                                                    v3 = v3 + &String::from(v2);
                                                    match init_itra.next() {
                                                        Some(next_char_value) => {
                                                            v2 = next_char_value
                                                        }
                                                        None => break,
                                                    }
                                                }
                                                if variant == "" {
                                                    let v4 = v.to_string()
                                                        + &d1.to_string()
                                                        + &d2.to_string()
                                                        + &v3;
                                                    effects.push((v4, 4));
                                                } else {
                                                    let v4 = variant.to_string()
                                                        + &v.to_string()
                                                        + &d1.to_string()
                                                        + &d2.to_string()
                                                        + &v3;
                                                    effects.push((v4, 4));
                                                }
                                            }
                                            _ => (),
                                        }
                                    }
                                    'a' => {
                                        let d2 = init_itra.next().unwrap();
                                        match d2 {
                                            's' => {
                                                //'basis'
                                                let mut v2 = init_itra.next().unwrap();
                                                let mut v3 = String::new();
                                                loop {
                                                    v3 = v3 + &String::from(v2);
                                                    match init_itra.next() {
                                                        Some(next_char_value) => {
                                                            v2 = next_char_value
                                                        }
                                                        None => break,
                                                    }
                                                }
                                                if variant == "" {
                                                    let v4 = v.to_string()
                                                        + &d1.to_string()
                                                        + &d2.to_string()
                                                        + &v3;
                                                    flex_grid.push((v4, 2));
                                                } else {
                                                    let v4 = variant.to_string()
                                                        + &v.to_string()
                                                        + &d1.to_string()
                                                        + &d2.to_string()
                                                        + &v3;
                                                    flex_grid.push((v4, 2));
                                                }
                                            }
                                            'c' => {
                                                //'backdrop'
                                                let mut v2 = init_itra.next().unwrap();
                                                let mut v3 = String::new();
                                                loop {
                                                    v3 = v3 + &String::from(v2);
                                                    match init_itra.next() {
                                                        Some(next_char_value) => {
                                                            v2 = next_char_value
                                                        }
                                                        None => break,
                                                    }
                                                }
                                                if variant == "" {
                                                    let v4 = v.to_string()
                                                        + &d1.to_string()
                                                        + &d2.to_string()
                                                        + &v3;
                                                    effects.push((v4, 11));
                                                } else {
                                                    let v4 = variant.to_string()
                                                        + &v.to_string()
                                                        + &d1.to_string()
                                                        + &d2.to_string()
                                                        + &v3;
                                                    effects.push((v4, 11));
                                                }
                                            }
                                            _ => (),
                                        }
                                    }
                                    'g' => {
                                        let d2 = init_itra.next().unwrap();
                                        match d2 {
                                            '-' => {
                                                let d3 = init_itra.next().unwrap();
                                                match d3 {
                                                    'g' => {
                                                        //'bg-gradient-to-'
                                                        let mut v2 = init_itra.next().unwrap();
                                                        let mut v3 = String::new();
                                                        loop {
                                                            v3 = v3 + &String::from(v2);
                                                            match init_itra.next() {
                                                                Some(next_char_value) => {
                                                                    v2 = next_char_value
                                                                }
                                                                None => break,
                                                            }
                                                        }
                                                        if variant == "" {
                                                            let v4 = v.to_string()
                                                                + &d1.to_string()
                                                                + &d2.to_string()
                                                                + &d3.to_string()
                                                                + &v3;
                                                            visuals.push((v4, 11));
                                                        } else {
                                                            let v4 = variant.to_string()
                                                                + &v.to_string()
                                                                + &d1.to_string()
                                                                + &d2.to_string()
                                                                + &d3.to_string()
                                                                + &v3;
                                                            visuals.push((v4, 11));
                                                        }
                                                    }
                                                    'n' => {
                                                        //'bg-none'
                                                        let mut v2 = init_itra.next().unwrap();
                                                        let mut v3 = String::new();
                                                        loop {
                                                            v3 = v3 + &String::from(v2);
                                                            match init_itra.next() {
                                                                Some(next_char_value) => {
                                                                    v2 = next_char_value
                                                                }
                                                                None => break,
                                                            }
                                                        }
                                                        if variant == "" {
                                                            let v4 = v.to_string()
                                                                + &d1.to_string()
                                                                + &d2.to_string()
                                                                + &d3.to_string()
                                                                + &v3;
                                                            visuals.push((v4, 11));
                                                        } else {
                                                            let v4 = variant.to_string()
                                                                + &v.to_string()
                                                                + &d1.to_string()
                                                                + &d2.to_string()
                                                                + &d3.to_string()
                                                                + &v3;
                                                            visuals.push((v4, 11));
                                                        }
                                                    }
                                                    'b' => {
                                                        //'bg-blend'
                                                        let mut v2 = init_itra.next().unwrap();
                                                        let mut v3 = String::new();
                                                        loop {
                                                            v3 = v3 + &String::from(v2);
                                                            match init_itra.next() {
                                                                Some(next_char_value) => {
                                                                    v2 = next_char_value
                                                                }
                                                                None => break,
                                                            }
                                                        }
                                                        if variant == "" {
                                                            let v4 = v.to_string()
                                                                + &d1.to_string()
                                                                + &d2.to_string()
                                                                + &d3.to_string()
                                                                + &v3;
                                                            effects.push((v4, 3));
                                                        } else {
                                                            let v4 = variant.to_string()
                                                                + &v.to_string()
                                                                + &d1.to_string()
                                                                + &d2.to_string()
                                                                + &d3.to_string()
                                                                + &v3;
                                                            effects.push((v4, 3));
                                                        }
                                                    }
                                                    _ => {
                                                        //'bg-'
                                                        let mut v2 = init_itra.next().unwrap();
                                                        let mut v3 = String::new();
                                                        loop {
                                                            v3 = v3 + &String::from(v2);
                                                            match init_itra.next() {
                                                                Some(next_char_value) => {
                                                                    v2 = next_char_value
                                                                }
                                                                None => break,
                                                            }
                                                        }
                                                        if variant == "" {
                                                            let v4 = v.to_string()
                                                                + &d1.to_string()
                                                                + &d2.to_string()
                                                                + &d3.to_string()
                                                                + &v3;
                                                            visuals.push((v4, 1));
                                                        } else {
                                                            let v4 = variant.to_string()
                                                                + &v.to_string()
                                                                + &d1.to_string()
                                                                + &d2.to_string()
                                                                + &d3.to_string()
                                                                + &v3;
                                                            visuals.push((v4, 1));
                                                        }
                                                    }
                                                }
                                            }
                                            _ => (),
                                        }
                                    }
                                    _ => (),
                                }
                            }
                            'c' => {
                                let d1 = init_itra.next().unwrap();
                                match d1 {
                                    'o' => {
                                        let d2 = init_itra.next().unwrap();
                                        match d2 {
                                            'n' => {
                                                let d3 = init_itra.next().unwrap();
                                                match d3 {
                                                    't' => {
                                                        let d4 = init_itra.next().unwrap();
                                                        match d4 {
                                                            'e' => {
                                                                let d5 = init_itra.next().unwrap();
                                                                match d5 {
                                                                    'n' => {
                                                                        let d6 = init_itra
                                                                            .next()
                                                                            .unwrap();
                                                                        match d6 {
                                                                            't' => {
                                                                                let d7 = init_itra
                                                                                    .next()
                                                                                    .unwrap();
                                                                                match d7 {
                                                                                    's' => {
                                                                                        //'contents'
                                                                                        let mut v2 = init_itra.next().unwrap();
                                                                                        let mut v3 = String::new();
                                                                                        loop {
                                                                                            v3 = v3 + &String::from(v2);
                                                                                            match init_itra.next() {
    																			        Some(next_char_value) => v2 = next_char_value,
    																			        None => break,
    																			    }
                                                                                        }
                                                                                        if variant
                                                                                            == ""
                                                                                        {
                                                                                            let v4 = v.to_string() + &d1.to_string()+ &d2.to_string()+ &d3.to_string()+ &d4.to_string()+ &d5.to_string()+ &d6.to_string()+ &d7.to_string() + &v3;
                                                                                            box_model.push((
																						v4,
    																			  		1,
    																				));
                                                                                        } else {
                                                                                            let v4 = variant.to_string() + &v.to_string() + &d1.to_string()+ &d2.to_string()+ &d3.to_string()+ &d4.to_string()+ &d5.to_string()+ &d6.to_string()+ &d7.to_string() + &v3;
                                                                                            box_model.push((
    																			  v4,
    																			  1,
    																			));
                                                                                        }
                                                                                    }
                                                                                    '-' => {
                                                                                        //'content-none'
                                                                                        let mut v2 = init_itra.next().unwrap();
                                                                                        let mut v3 = String::new();
                                                                                        loop {
                                                                                            v3 = v3 + &String::from(v2);
                                                                                            match init_itra.next() {
    																			        Some(next_char_value) => v2 = next_char_value,
    																			        None => break,
    																			    }
                                                                                        }
                                                                                        if variant
                                                                                            == ""
                                                                                        {
                                                                                            let v4 = v.to_string() + &d1.to_string()+ &d2.to_string()+ &d3.to_string()+ &d4.to_string()+ &d5.to_string()+ &d6.to_string()+ &d7.to_string() + &v3;
                                                                                            typography.push((
																						v4,
    																			  		16,
    																				));
                                                                                        } else {
                                                                                            let v4 = variant.to_string() + &v.to_string() + &d1.to_string()+ &d2.to_string()+ &d3.to_string()+ &d4.to_string()+ &d5.to_string()+ &d6.to_string()+ &d7.to_string() + &v3;
                                                                                            typography.push((
    																			  v4,
    																			  16,
    																			));
                                                                                        }
                                                                                    }
                                                                                    _ => {
                                                                                        //'content'
                                                                                        let mut v2 = init_itra.next().unwrap();
                                                                                        let mut v3 = String::new();
                                                                                        loop {
                                                                                            v3 = v3 + &String::from(v2);
                                                                                            match init_itra.next() {
    																			        Some(next_char_value) => v2 = next_char_value,
    																			        None => break,
    																			    }
                                                                                        }
                                                                                        if variant
                                                                                            == ""
                                                                                        {
                                                                                            let v4 = v.to_string() + &d1.to_string()+ &d2.to_string()+ &d3.to_string()+ &d4.to_string()+ &d5.to_string()+ &d6.to_string()+ &d7.to_string() + &v3;
                                                                                            alignment.push((
																						v4,
    																			  		8,
    																				));
                                                                                        } else {
                                                                                            let v4 = variant.to_string() + &v.to_string() + &d1.to_string()+ &d2.to_string()+ &d3.to_string()+ &d4.to_string()+ &d5.to_string()+ &d6.to_string()+ &d7.to_string() + &v3;
                                                                                            alignment.push((
    																			  v4,
    																			  8,
    																			));
                                                                                        }
                                                                                    }
                                                                                }
                                                                            }
                                                                            _ => (),
                                                                        }
                                                                    }
                                                                    _ => (),
                                                                }
                                                            }
                                                            'a' => {
                                                                //'container'
                                                                let mut v2 =
                                                                    init_itra.next().unwrap();
                                                                let mut v3 = String::new();
                                                                loop {
                                                                    v3 = v3 + &String::from(v2);
                                                                    match init_itra.next() {
                                                                        Some(next_char_value) => {
                                                                            v2 = next_char_value
                                                                        }
                                                                        None => break,
                                                                    }
                                                                }
                                                                if variant == "" {
                                                                    let v4 = v.to_string()
                                                                        + &d1.to_string()
                                                                        + &d2.to_string()
                                                                        + &d3.to_string()
                                                                        + &d4.to_string()
                                                                        + &v3;
                                                                    box_model.push((v4, 2));
                                                                } else {
                                                                    let v4 = variant.to_string()
                                                                        + &v.to_string()
                                                                        + &d1.to_string()
                                                                        + &d2.to_string()
                                                                        + &d3.to_string()
                                                                        + &d4.to_string()
                                                                        + &v3;
                                                                    box_model.push((v4, 2));
                                                                }
                                                            }
                                                            'r' => {
                                                                //'contrast'
                                                                let mut v2 =
                                                                    init_itra.next().unwrap();
                                                                let mut v3 = String::new();
                                                                loop {
                                                                    v3 = v3 + &String::from(v2);
                                                                    match init_itra.next() {
                                                                        Some(next_char_value) => {
                                                                            v2 = next_char_value
                                                                        }
                                                                        None => break,
                                                                    }
                                                                }
                                                                if variant == "" {
                                                                    let v4 = v.to_string()
                                                                        + &d1.to_string()
                                                                        + &d2.to_string()
                                                                        + &d3.to_string()
                                                                        + &d4.to_string()
                                                                        + &v3;
                                                                    effects.push((v4, 4));
                                                                } else {
                                                                    let v4 = variant.to_string()
                                                                        + &v.to_string()
                                                                        + &d1.to_string()
                                                                        + &d2.to_string()
                                                                        + &d3.to_string()
                                                                        + &d4.to_string()
                                                                        + &v3;
                                                                    effects.push((v4, 4));
                                                                }
                                                            }
                                                            _ => (),
                                                        }
                                                    }
                                                    _ => (),
                                                }
                                            }
                                            'l' => {
                                                let d3 = init_itra.next().unwrap();
                                                match d3 {
                                                    'u' => {
                                                        //'columns'
                                                        let mut v2 = init_itra.next().unwrap();
                                                        let mut v3 = String::new();
                                                        loop {
                                                            v3 = v3 + &String::from(v2);
                                                            match init_itra.next() {
                                                                Some(next_char_value) => {
                                                                    v2 = next_char_value
                                                                }
                                                                None => break,
                                                            }
                                                        }
                                                        if variant == "" {
                                                            let v4 = v.to_string()
                                                                + &d1.to_string()
                                                                + &d2.to_string()
                                                                + &d3.to_string()
                                                                + &v3;
                                                            box_model.push((v4, 3));
                                                        } else {
                                                            let v4 = variant.to_string()
                                                                + &v.to_string()
                                                                + &d1.to_string()
                                                                + &d2.to_string()
                                                                + &d3.to_string()
                                                                + &v3;
                                                            box_model.push((v4, 3));
                                                        }
                                                    }
                                                    's' => {
                                                        //'cols'
                                                        let mut v2 = init_itra.next().unwrap();
                                                        let mut v3 = String::new();
                                                        loop {
                                                            v3 = v3 + &String::from(v2);
                                                            match init_itra.next() {
                                                                Some(next_char_value) => {
                                                                    v2 = next_char_value
                                                                }
                                                                None => break,
                                                            }
                                                        }
                                                        if variant == "" {
                                                            let v4 = v.to_string()
                                                                + &d1.to_string()
                                                                + &d2.to_string()
                                                                + &d3.to_string()
                                                                + &v3;
                                                            flex_grid.push((v4, 2));
                                                        } else {
                                                            let v4 = variant.to_string()
                                                                + &v.to_string()
                                                                + &d1.to_string()
                                                                + &d2.to_string()
                                                                + &d3.to_string()
                                                                + &v3;
                                                            flex_grid.push((v4, 2));
                                                        }
                                                    }
                                                    _ => (),
                                                }
                                            }
                                            _ => (),
                                        }
                                    }
                                    'l' => {
                                        //'clear'
                                        let mut v2 = init_itra.next().unwrap();
                                        let mut v3 = String::new();
                                        loop {
                                            v3 = v3 + &String::from(v2);
                                            match init_itra.next() {
                                                Some(next_char_value) => v2 = next_char_value,
                                                None => break,
                                            }
                                        }
                                        if variant == "" {
                                            let v4 = v.to_string() + &d1.to_string() + &v3;
                                            box_model.push((v4, 10));
                                        } else {
                                            let v4 = variant.to_string()
                                                + &v.to_string()
                                                + &d1.to_string()
                                                + &v3;
                                            box_model.push((v4, 10));
                                        }
                                    }
                                    'a' => {
                                        let d2 = init_itra.next().unwrap();
                                        match d2 {
                                            'p' => {
                                                //'capitalize'
                                                let mut v2 = init_itra.next().unwrap();
                                                let mut v3 = String::new();
                                                loop {
                                                    v3 = v3 + &String::from(v2);
                                                    match init_itra.next() {
                                                        Some(next_char_value) => {
                                                            v2 = next_char_value
                                                        }
                                                        None => break,
                                                    }
                                                }
                                                if variant == "" {
                                                    let v4 = v.to_string()
                                                        + &d1.to_string()
                                                        + &d2.to_string()
                                                        + &v3;
                                                    typography.push((v4, 11));
                                                } else {
                                                    let v4 = variant.to_string()
                                                        + &v.to_string()
                                                        + &d1.to_string()
                                                        + &d2.to_string()
                                                        + &v3;
                                                    typography.push((v4, 11));
                                                }
                                            }
                                            'r' => {
                                                //'caret'
                                                let mut v2 = init_itra.next().unwrap();
                                                let mut v3 = String::new();
                                                loop {
                                                    v3 = v3 + &String::from(v2);
                                                    match init_itra.next() {
                                                        Some(next_char_value) => {
                                                            v2 = next_char_value
                                                        }
                                                        None => break,
                                                    }
                                                }
                                                if variant == "" {
                                                    let v4 = v.to_string()
                                                        + &d1.to_string()
                                                        + &d2.to_string()
                                                        + &v3;
                                                    misc.push((v4, 1));
                                                } else {
                                                    let v4 = variant.to_string()
                                                        + &v.to_string()
                                                        + &d1.to_string()
                                                        + &d2.to_string()
                                                        + &v3;
                                                    misc.push((v4, 1));
                                                }
                                            }
                                            _ => (),
                                        }
                                    }
                                    'u' => {
                                        //'cursor'
                                        let mut v2 = init_itra.next().unwrap();
                                        let mut v3 = String::new();
                                        loop {
                                            v3 = v3 + &String::from(v2);
                                            match init_itra.next() {
                                                Some(next_char_value) => v2 = next_char_value,
                                                None => break,
                                            }
                                        }
                                        if variant == "" {
                                            let v4 = v.to_string() + &d1.to_string() + &v3;
                                            misc.push((v4, 1));
                                        } else {
                                            let v4 = variant.to_string()
                                                + &v.to_string()
                                                + &d1.to_string()
                                                + &v3;
                                            misc.push((v4, 1));
                                        }
                                    }
                                    _ => (),
                                }
                            }
                            'd' => {
                                let d1 = init_itra.next().unwrap();
                                match d1 {
                                    'i' => {
                                        let d2 = init_itra.next().unwrap();
                                        match d2 {
                                            'v' => {
                                                //'divide'
                                                let mut v2 = init_itra.next().unwrap();
                                                let mut v3 = String::new();
                                                loop {
                                                    v3 = v3 + &String::from(v2);
                                                    match init_itra.next() {
                                                        Some(next_char_value) => {
                                                            v2 = next_char_value
                                                        }
                                                        None => break,
                                                    }
                                                }
                                                if variant == "" {
                                                    let v4 = v.to_string()
                                                        + &d1.to_string()
                                                        + &d2.to_string()
                                                        + &v3;
                                                    box_model.push((v4, 21));
                                                } else {
                                                    let v4 = variant.to_string()
                                                        + &v.to_string()
                                                        + &d1.to_string()
                                                        + &d2.to_string()
                                                        + &v3;
                                                    box_model.push((v4, 21));
                                                }
                                            }
                                            'a' => {
                                                //'diagonal-fractions'
                                                let mut v2 = init_itra.next().unwrap();
                                                let mut v3 = String::new();
                                                loop {
                                                    v3 = v3 + &String::from(v2);
                                                    match init_itra.next() {
                                                        Some(next_char_value) => {
                                                            v2 = next_char_value
                                                        }
                                                        None => break,
                                                    }
                                                }
                                                if variant == "" {
                                                    let v4 = v.to_string()
                                                        + &d1.to_string()
                                                        + &d2.to_string()
                                                        + &v3;
                                                    typography.push((v4, 9));
                                                } else {
                                                    let v4 = variant.to_string()
                                                        + &v.to_string()
                                                        + &d1.to_string()
                                                        + &d2.to_string()
                                                        + &v3;
                                                    typography.push((v4, 9));
                                                }
                                            }
                                            _ => (),
                                        }
                                    }
                                    'e' => {
                                        let d2 = init_itra.next().unwrap();
                                        match d2 {
                                            'c' => {
                                                //'decoration'
                                                let mut v2 = init_itra.next().unwrap();
                                                let mut v3 = String::new();
                                                loop {
                                                    v3 = v3 + &String::from(v2);
                                                    match init_itra.next() {
                                                        Some(next_char_value) => {
                                                            v2 = next_char_value
                                                        }
                                                        None => break,
                                                    }
                                                }
                                                if variant == "" {
                                                    let v4 = v.to_string()
                                                        + &d1.to_string()
                                                        + &d2.to_string()
                                                        + &v3;
                                                    typography.push((v4, 10));
                                                } else {
                                                    let v4 = variant.to_string()
                                                        + &v.to_string()
                                                        + &d1.to_string()
                                                        + &d2.to_string()
                                                        + &v3;
                                                    typography.push((v4, 10));
                                                }
                                            }
                                            'l' => {
                                                //'delay'
                                                let mut v2 = init_itra.next().unwrap();
                                                let mut v3 = String::new();
                                                loop {
                                                    v3 = v3 + &String::from(v2);
                                                    match init_itra.next() {
                                                        Some(next_char_value) => {
                                                            v2 = next_char_value
                                                        }
                                                        None => break,
                                                    }
                                                }
                                                if variant == "" {
                                                    let v4 = v.to_string()
                                                        + &d1.to_string()
                                                        + &d2.to_string()
                                                        + &v3;
                                                    motion.push((v4, 4));
                                                } else {
                                                    let v4 = variant.to_string()
                                                        + &v.to_string()
                                                        + &d1.to_string()
                                                        + &d2.to_string()
                                                        + &v3;
                                                    motion.push((v4, 4));
                                                }
                                            }
                                            _ => (),
                                        }
                                    }
                                    'r' => {
                                        //'drop-shadow'
                                        let mut v2 = init_itra.next().unwrap();
                                        let mut v3 = String::new();
                                        loop {
                                            v3 = v3 + &String::from(v2);
                                            match init_itra.next() {
                                                Some(next_char_value) => v2 = next_char_value,
                                                None => break,
                                            }
                                        }
                                        if variant == "" {
                                            let v4 = v.to_string() + &d1.to_string() + &v3;
                                            effects.push((v4, 5));
                                        } else {
                                            let v4 = variant.to_string()
                                                + &v.to_string()
                                                + &d1.to_string()
                                                + &v3;
                                            effects.push((v4, 5));
                                        }
                                    }
                                    'u' => {
                                        //'duration'
                                        let mut v2 = init_itra.next().unwrap();
                                        let mut v3 = String::new();
                                        loop {
                                            v3 = v3 + &String::from(v2);
                                            match init_itra.next() {
                                                Some(next_char_value) => v2 = next_char_value,
                                                None => break,
                                            }
                                        }
                                        if variant == "" {
                                            let v4 = v.to_string() + &d1.to_string() + &v3;
                                            motion.push((v4, 3));
                                        } else {
                                            let v4 = variant.to_string()
                                                + &v.to_string()
                                                + &d1.to_string()
                                                + &v3;
                                            motion.push((v4, 3));
                                        }
                                    }
                                    _ => (),
                                }
                            }
                            'e' => {
                                let d1 = init_itra.next().unwrap();
                                match d1 {
                                    'a' => {
                                        let d2 = init_itra.next().unwrap();
                                        match d2 {
                                            's' => {
                                                let d3 = init_itra.next().unwrap();
                                                match d3 {
                                                    'e' => {
                                                        let d4 = init_itra.next().unwrap();
                                                        match d4 {
                                                            '-' => {
                                                                let d5 = init_itra.next().unwrap();
                                                                match d5 {
                                                                    'l' => {
                                                                        //'ease-linear'
                                                                        let mut v2 = init_itra
                                                                            .next()
                                                                            .unwrap();
                                                                        let mut v3 = String::new();
                                                                        loop {
                                                                            v3 = v3
                                                                                + &String::from(v2);
                                                                            match init_itra.next() {
                                                                                Some(
                                                                                    next_char_value,
                                                                                ) => v2 =
                                                                                    next_char_value,
                                                                                None => break,
                                                                            }
                                                                        }
                                                                        if variant == "" {
                                                                            let v4 = v.to_string()
                                                                                + &d1.to_string()
                                                                                + &d2.to_string()
                                                                                + &d3.to_string()
                                                                                + &d4.to_string()
                                                                                + &d5.to_string()
                                                                                + &v3;
                                                                            motion.push((v4, 5));
                                                                        } else {
                                                                            let v4 = variant
                                                                                .to_string()
                                                                                + &v.to_string()
                                                                                + &d1.to_string()
                                                                                + &d2.to_string()
                                                                                + &d3.to_string()
                                                                                + &d4.to_string()
                                                                                + &d5.to_string()
                                                                                + &v3;
                                                                            motion.push((v4, 5));
                                                                        }
                                                                    }
                                                                    'i' => {
                                                                        //'ease-in'
                                                                        let mut v2 = init_itra
                                                                            .next()
                                                                            .unwrap();
                                                                        let mut v3 = String::new();
                                                                        loop {
                                                                            v3 = v3
                                                                                + &String::from(v2);
                                                                            match init_itra.next() {
                                                                                Some(
                                                                                    next_char_value,
                                                                                ) => v2 =
                                                                                    next_char_value,
                                                                                None => break,
                                                                            }
                                                                        }
                                                                        if variant == "" {
                                                                            let v4 = v.to_string()
                                                                                + &d1.to_string()
                                                                                + &d2.to_string()
                                                                                + &d3.to_string()
                                                                                + &d4.to_string()
                                                                                + &d5.to_string()
                                                                                + &v3;
                                                                            motion.push((v4, 5));
                                                                        } else {
                                                                            let v4 = variant
                                                                                .to_string()
                                                                                + &v.to_string()
                                                                                + &d1.to_string()
                                                                                + &d2.to_string()
                                                                                + &d3.to_string()
                                                                                + &d4.to_string()
                                                                                + &d5.to_string()
                                                                                + &v3;
                                                                            motion.push((v4, 5));
                                                                        }
                                                                    }
                                                                    'o' => {
                                                                        //'ease-out'
                                                                        let mut v2 = init_itra
                                                                            .next()
                                                                            .unwrap();
                                                                        let mut v3 = String::new();
                                                                        loop {
                                                                            v3 = v3
                                                                                + &String::from(v2);
                                                                            match init_itra.next() {
                                                                                Some(
                                                                                    next_char_value,
                                                                                ) => v2 =
                                                                                    next_char_value,
                                                                                None => break,
                                                                            }
                                                                        }
                                                                        if variant == "" {
                                                                            let v4 = v.to_string()
                                                                                + &d1.to_string()
                                                                                + &d2.to_string()
                                                                                + &d3.to_string()
                                                                                + &d4.to_string()
                                                                                + &d5.to_string()
                                                                                + &v3;
                                                                            motion.push((v4, 5));
                                                                        } else {
                                                                            let v4 = variant
                                                                                .to_string()
                                                                                + &v.to_string()
                                                                                + &d1.to_string()
                                                                                + &d2.to_string()
                                                                                + &d3.to_string()
                                                                                + &d4.to_string()
                                                                                + &d5.to_string()
                                                                                + &v3;
                                                                            motion.push((v4, 5));
                                                                        }
                                                                    }
                                                                    _ => (),
                                                                }
                                                            }
                                                            _ => (),
                                                        }
                                                    }
                                                    _ => (),
                                                }
                                            }
                                            _ => (),
                                        }
                                    }
                                    _ => (),
                                }
                            }
                            'f' => {
                                let d1 = init_itra.next().unwrap();
                                match d1 {
                                    'i' => {
                                        let d2 = init_itra.next().unwrap();
                                        match d2 {
                                            'x' => {
                                                //'fixed'
                                                let mut v2 = init_itra.next().unwrap();
                                                let mut v3 = String::new();
                                                loop {
                                                    v3 = v3 + &String::from(v2);
                                                    match init_itra.next() {
                                                        Some(next_char_value) => {
                                                            v2 = next_char_value
                                                        }
                                                        None => break,
                                                    }
                                                }
                                                if variant == "" {
                                                    let v4 = v.to_string()
                                                        + &d1.to_string()
                                                        + &d2.to_string()
                                                        + &v3;
                                                    positioning.push((v4, 1));
                                                } else {
                                                    let v4 = variant.to_string()
                                                        + &v.to_string()
                                                        + &d1.to_string()
                                                        + &d2.to_string()
                                                        + &v3;
                                                    positioning.push((v4, 1));
                                                }
                                            }
                                            'l' => {
                                                //'fill'
                                                let mut v2 = init_itra.next().unwrap();
                                                let mut v3 = String::new();
                                                loop {
                                                    v3 = v3 + &String::from(v2);
                                                    match init_itra.next() {
                                                        Some(next_char_value) => {
                                                            v2 = next_char_value
                                                        }
                                                        None => break,
                                                    }
                                                }
                                                if variant == "" {
                                                    let v4 = v.to_string()
                                                        + &d1.to_string()
                                                        + &d2.to_string()
                                                        + &v3;
                                                    visuals.push((v4, 1));
                                                } else {
                                                    let v4 = variant.to_string()
                                                        + &v.to_string()
                                                        + &d1.to_string()
                                                        + &d2.to_string()
                                                        + &v3;
                                                    visuals.push((v4, 1));
                                                }
                                            }
                                            _ => (),
                                        }
                                    }
                                    'l' => {
                                        let d2 = init_itra.next().unwrap();
                                        match d2 {
                                            'e' => {
                                                //'flex'
                                                let mut v2 = init_itra.next().unwrap();
                                                let mut v3 = String::new();
                                                loop {
                                                    v3 = v3 + &String::from(v2);
                                                    match init_itra.next() {
                                                        Some(next_char_value) => {
                                                            v2 = next_char_value
                                                        }
                                                        None => break,
                                                    }
                                                }
                                                if variant == "" {
                                                    let v4 = v.to_string()
                                                        + &d1.to_string()
                                                        + &d2.to_string()
                                                        + &v3;
                                                    box_model.push((v4, 1));
                                                } else {
                                                    let v4 = variant.to_string()
                                                        + &v.to_string()
                                                        + &d1.to_string()
                                                        + &d2.to_string()
                                                        + &v3;
                                                    box_model.push((v4, 1));
                                                }
                                            }
                                            'o' => {
                                                let d3 = init_itra.next().unwrap();
                                                match d3 {
                                                    'w' => {
                                                        //'flow-root'
                                                        let mut v2 = init_itra.next().unwrap();
                                                        let mut v3 = String::new();
                                                        loop {
                                                            v3 = v3 + &String::from(v2);
                                                            match init_itra.next() {
                                                                Some(next_char_value) => {
                                                                    v2 = next_char_value
                                                                }
                                                                None => break,
                                                            }
                                                        }
                                                        if variant == "" {
                                                            let v4 = v.to_string()
                                                                + &d1.to_string()
                                                                + &d2.to_string()
                                                                + &d3.to_string()
                                                                + &v3;
                                                            box_model.push((v4, 1));
                                                        } else {
                                                            let v4 = variant.to_string()
                                                                + &v.to_string()
                                                                + &d1.to_string()
                                                                + &d2.to_string()
                                                                + &d3.to_string()
                                                                + &v3;
                                                            box_model.push((v4, 1));
                                                        }
                                                    }
                                                    'a' => {
                                                        //'float'
                                                        let mut v2 = init_itra.next().unwrap();
                                                        let mut v3 = String::new();
                                                        loop {
                                                            v3 = v3 + &String::from(v2);
                                                            match init_itra.next() {
                                                                Some(next_char_value) => {
                                                                    v2 = next_char_value
                                                                }
                                                                None => break,
                                                            }
                                                        }
                                                        if variant == "" {
                                                            let v4 = v.to_string()
                                                                + &d1.to_string()
                                                                + &d2.to_string()
                                                                + &d3.to_string()
                                                                + &v3;
                                                            box_model.push((v4, 10));
                                                        } else {
                                                            let v4 = variant.to_string()
                                                                + &v.to_string()
                                                                + &d1.to_string()
                                                                + &d2.to_string()
                                                                + &d3.to_string()
                                                                + &v3;
                                                            box_model.push((v4, 10));
                                                        }
                                                    }
                                                    _ => (),
                                                }
                                            }
                                            _ => (),
                                        }
                                    }
                                    'o' => {
                                        //'font-'
                                        let mut v2 = init_itra.next().unwrap();
                                        let mut v3 = String::new();
                                        loop {
                                            v3 = v3 + &String::from(v2);
                                            match init_itra.next() {
                                                Some(next_char_value) => v2 = next_char_value,
                                                None => break,
                                            }
                                        }
                                        if variant == "" {
                                            let v4 = v.to_string() + &d1.to_string() + &v3;
                                            typography.push((v4, 1));
                                        } else {
                                            let v4 = variant.to_string()
                                                + &v.to_string()
                                                + &d1.to_string()
                                                + &v3;
                                            typography.push((v4, 1));
                                        }
                                    }
                                    'r' => {
                                        //'from-'
                                        let mut v2 = init_itra.next().unwrap();
                                        let mut v3 = String::new();
                                        loop {
                                            v3 = v3 + &String::from(v2);
                                            match init_itra.next() {
                                                Some(next_char_value) => v2 = next_char_value,
                                                None => break,
                                            }
                                        }
                                        if variant == "" {
                                            let v4 = v.to_string() + &d1.to_string() + &v3;
                                            visuals.push((v4, 12));
                                        } else {
                                            let v4 = variant.to_string()
                                                + &v.to_string()
                                                + &d1.to_string()
                                                + &v3;
                                            visuals.push((v4, 12));
                                        }
                                    }
                                    _ => (),
                                }
                            }
                            'g' => {
                                let d1 = init_itra.next().unwrap();
                                match d1 {
                                    'r' => {
                                        let d2 = init_itra.next().unwrap();
                                        match d2 {
                                            'i' => {
                                                let d3 = init_itra.next().unwrap();
                                                match d3 {
                                                    'd' => {
                                                        let d4 = init_itra.next().unwrap_or_default();
                                                        match d4 {
                                                            '-' => {
                                                                let d5 = init_itra.next().unwrap();
                                                                match d5 {
                                                                    'r' => {
                                                                        //'grid-rows'
                                                                        let mut v2 = init_itra
                                                                            .next()
                                                                            .unwrap();
                                                                        let mut v3 = String::new();
                                                                        loop {
                                                                            v3 = v3
                                                                                + &String::from(v2);
                                                                            match init_itra.next() {
                                                                                Some(
                                                                                    next_char_value,
                                                                                ) => v2 =
                                                                                    next_char_value,
                                                                                None => break,
                                                                            }
                                                                        }
                                                                        if variant == "" {
                                                                            let v4 = v.to_string()
                                                                                + &d1.to_string()
                                                                                + &d2.to_string()
                                                                                + &d3.to_string()
                                                                                + &d4.to_string()
                                                                                + &d5.to_string()
                                                                                + &v3;
                                                                            flex_grid.push((v4, 3));
                                                                        } else {
                                                                            let v4 = variant
                                                                                .to_string()
                                                                                + &v.to_string()
                                                                                + &d1.to_string()
                                                                                + &d2.to_string()
                                                                                + &d3.to_string()
                                                                                + &d4.to_string()
                                                                                + &d5.to_string()
                                                                                + &v3;
                                                                            flex_grid.push((v4, 3));
                                                                        }
                                                                    }
                                                                    'c' => {
                                                                        //'grid-cols'
                                                                        let mut v2 = init_itra
                                                                            .next()
                                                                            .unwrap();
                                                                        let mut v3 = String::new();
                                                                        loop {
                                                                            v3 = v3
                                                                                + &String::from(v2);
                                                                            match init_itra.next() {
                                                                                Some(
                                                                                    next_char_value,
                                                                                ) => v2 =
                                                                                    next_char_value,
                                                                                None => break,
                                                                            }
                                                                        }
                                                                        if variant == "" {
                                                                            let v4 = v.to_string()
                                                                                + &d1.to_string()
                                                                                + &d2.to_string()
                                                                                + &d3.to_string()
                                                                                + &d4.to_string()
                                                                                + &d5.to_string()
                                                                                + &v3;
                                                                            flex_grid.push((v4, 4));
                                                                        } else {
                                                                            let v4 = variant
                                                                                .to_string()
                                                                                + &v.to_string()
                                                                                + &d1.to_string()
                                                                                + &d2.to_string()
                                                                                + &d3.to_string()
                                                                                + &d4.to_string()
                                                                                + &d5.to_string()
                                                                                + &v3;
                                                                            flex_grid.push((v4, 4));
                                                                        }
                                                                    }
                                                                    'f' => {
                                                                        //'grid-flow'
                                                                        let mut v2 = init_itra
                                                                            .next()
                                                                            .unwrap();
                                                                        let mut v3 = String::new();
                                                                        loop {
                                                                            v3 = v3
                                                                                + &String::from(v2);
                                                                            match init_itra.next() {
                                                                                Some(
                                                                                    next_char_value,
                                                                                ) => v2 =
                                                                                    next_char_value,
                                                                                None => break,
                                                                            }
                                                                        }
                                                                        if variant == "" {
                                                                            let v4 = v.to_string()
                                                                                + &d1.to_string()
                                                                                + &d2.to_string()
                                                                                + &d3.to_string()
                                                                                + &d4.to_string()
                                                                                + &d5.to_string()
                                                                                + &v3;
                                                                            flex_grid.push((v4, 5));
                                                                        } else {
                                                                            let v4 = variant
                                                                                .to_string()
                                                                                + &v.to_string()
                                                                                + &d1.to_string()
                                                                                + &d2.to_string()
                                                                                + &d3.to_string()
                                                                                + &d4.to_string()
                                                                                + &d5.to_string()
                                                                                + &v3;
                                                                            flex_grid.push((v4, 5));
                                                                        }
                                                                    }
                                                                    _ => (),
                                                                }
                                                            }
                                                            _ => {
                                                                //'grid'
                                                                if variant == "" {
                                                                    let v4 = v.to_string()
                                                                        + &d1.to_string()
                                                                        + &d2.to_string()
                                                                        + &d3.to_string()
                                                                        + &d4.to_string();
                                                                    box_model.push((v4, 1));
                                                                } else {
                                                                    let v4 = variant.to_string()
                                                                        + &v.to_string()
                                                                        + &d1.to_string()
                                                                        + &d2.to_string()
                                                                        + &d3.to_string()
                                                                        + &d4.to_string();
                                                                    box_model.push((v4, 1));
                                                                }
                                                            }
                                                        }
                                                    }
                                                    _ => (),
                                                }
                                            }
                                            'o' => {
                                                //'grow'
                                                let mut v2 = init_itra.next().unwrap();
                                                let mut v3 = String::new();
                                                loop {
                                                    v3 = v3 + &String::from(v2);
                                                    match init_itra.next() {
                                                        Some(next_char_value) => {
                                                            v2 = next_char_value
                                                        }
                                                        None => break,
                                                    }
                                                }
                                                if variant == "" {
                                                    let v4 = v.to_string()
                                                        + &d1.to_string()
                                                        + &d2.to_string()
                                                        + &v3;
                                                    flex_grid.push((v4, 2));
                                                } else {
                                                    let v4 = variant.to_string()
                                                        + &v.to_string()
                                                        + &d1.to_string()
                                                        + &d2.to_string()
                                                        + &v3;
                                                    flex_grid.push((v4, 2));
                                                }
                                            }
                                            'a' => {
                                                //'grayscale'
                                                let mut v2 = init_itra.next().unwrap();
                                                let mut v3 = String::new();
                                                loop {
                                                    v3 = v3 + &String::from(v2);
                                                    match init_itra.next() {
                                                        Some(next_char_value) => {
                                                            v2 = next_char_value
                                                        }
                                                        None => break,
                                                    }
                                                }
                                                if variant == "" {
                                                    let v4 = v.to_string()
                                                        + &d1.to_string()
                                                        + &d2.to_string()
                                                        + &v3;
                                                    effects.push((v4, 6));
                                                } else {
                                                    let v4 = variant.to_string()
                                                        + &v.to_string()
                                                        + &d1.to_string()
                                                        + &d2.to_string()
                                                        + &v3;
                                                    effects.push((v4, 6));
                                                }
                                            }
                                            _ => (),
                                        }
                                    }
                                    'a' => {
                                        let d2 = init_itra.next().unwrap();
                                        match d2 {
                                            'p' => {
                                                let d3 = init_itra.next().unwrap();
                                                match d3 {
                                                    '-' => {
                                                        let d4 = init_itra.next().unwrap();
                                                        match d4 {
                                                            'x' => {
                                                                //'gap-x'
                                                                let mut v2 =
                                                                    init_itra.next().unwrap();
                                                                let mut v3 = String::new();
                                                                loop {
                                                                    v3 = v3 + &String::from(v2);
                                                                    match init_itra.next() {
                                                                        Some(next_char_value) => {
                                                                            v2 = next_char_value
                                                                        }
                                                                        None => break,
                                                                    }
                                                                }
                                                                if variant == "" {
                                                                    let v4 = v.to_string()
                                                                        + &d1.to_string()
                                                                        + &d2.to_string()
                                                                        + &d3.to_string()
                                                                        + &d4.to_string()
                                                                        + &v3;
                                                                    flex_grid.push((v4, 9));
                                                                } else {
                                                                    let v4 = variant.to_string()
                                                                        + &v.to_string()
                                                                        + &d1.to_string()
                                                                        + &d2.to_string()
                                                                        + &d3.to_string()
                                                                        + &d4.to_string()
                                                                        + &v3;
                                                                    flex_grid.push((v4, 9));
                                                                }
                                                            }
                                                            'y' => {
                                                                //'gap-y'
                                                                let mut v2 =
                                                                    init_itra.next().unwrap();
                                                                let mut v3 = String::new();
                                                                loop {
                                                                    v3 = v3 + &String::from(v2);
                                                                    match init_itra.next() {
                                                                        Some(next_char_value) => {
                                                                            v2 = next_char_value
                                                                        }
                                                                        None => break,
                                                                    }
                                                                }
                                                                if variant == "" {
                                                                    let v4 = v.to_string()
                                                                        + &d1.to_string()
                                                                        + &d2.to_string()
                                                                        + &d3.to_string()
                                                                        + &d4.to_string()
                                                                        + &v3;
                                                                    flex_grid.push((v4, 10));
                                                                } else {
                                                                    let v4 = variant.to_string()
                                                                        + &v.to_string()
                                                                        + &d1.to_string()
                                                                        + &d2.to_string()
                                                                        + &d3.to_string()
                                                                        + &d4.to_string()
                                                                        + &v3;
                                                                    flex_grid.push((v4, 10));
                                                                }
                                                            }
                                                            _ => {
                                                                //'gap-'
                                                                let mut v2 =
                                                                    init_itra.next().unwrap();
                                                                let mut v3 = String::new();
                                                                loop {
                                                                    v3 = v3 + &String::from(v2);
                                                                    match init_itra.next() {
                                                                        Some(next_char_value) => {
                                                                            v2 = next_char_value
                                                                        }
                                                                        None => break,
                                                                    }
                                                                }
                                                                if variant == "" {
                                                                    let v4 = v.to_string()
                                                                        + &d1.to_string()
                                                                        + &d2.to_string()
                                                                        + &d3.to_string()
                                                                        + &d4.to_string()
                                                                        + &v3;
                                                                    flex_grid.push((v4, 8));
                                                                } else {
                                                                    let v4 = variant.to_string()
                                                                        + &v.to_string()
                                                                        + &d1.to_string()
                                                                        + &d2.to_string()
                                                                        + &d3.to_string()
                                                                        + &d4.to_string()
                                                                        + &v3;
                                                                    flex_grid.push((v4, 8));
                                                                }
                                                            }
                                                        }
                                                    }
                                                    _ => (),
                                                }
                                            }
                                            _ => (),
                                        }
                                    }
                                    _ => (),
                                }
                            }
                            'h' => {
                                let d1 = init_itra.next().unwrap();
                                match d1 {
                                    'i' => {
                                        //'hidden'
                                        let mut v2 = init_itra.next().unwrap();
                                        let mut v3 = String::new();
                                        loop {
                                            v3 = v3 + &String::from(v2);
                                            match init_itra.next() {
                                                Some(next_char_value) => v2 = next_char_value,
                                                None => break,
                                            }
                                        }
                                        if variant == "" {
                                            let v4 = v.to_string() + &d1.to_string() + &v3;
                                            box_model.push((v4, 1));
                                        } else {
                                            let v4 = variant.to_string()
                                                + &v.to_string()
                                                + &d1.to_string()
                                                + &v3;
                                            box_model.push((v4, 1));
                                        }
                                    }
                                    '-' => {
                                        //'h-'
                                        let mut v2 = init_itra.next().unwrap();
                                        let mut v3 = String::new();
                                        loop {
                                            v3 = v3 + &String::from(v2);
                                            match init_itra.next() {
                                                Some(next_char_value) => v2 = next_char_value,
                                                None => break,
                                            }
                                        }
                                        if variant == "" {
                                            let v4 = v.to_string() + &d1.to_string() + &v3;
                                            box_model.push((v4, 7));
                                        } else {
                                            let v4 = variant.to_string()
                                                + &v.to_string()
                                                + &d1.to_string()
                                                + &v3;
                                            box_model.push((v4, 7));
                                        }
                                    }
                                    'u' => {
                                        //'hue'
                                        let mut v2 = init_itra.next().unwrap();
                                        let mut v3 = String::new();
                                        loop {
                                            v3 = v3 + &String::from(v2);
                                            match init_itra.next() {
                                                Some(next_char_value) => v2 = next_char_value,
                                                None => break,
                                            }
                                        }
                                        if variant == "" {
                                            let v4 = v.to_string() + &d1.to_string() + &v3;
                                            effects.push((v4, 7));
                                        } else {
                                            let v4 = variant.to_string()
                                                + &v.to_string()
                                                + &d1.to_string()
                                                + &v3;
                                            effects.push((v4, 7));
                                        }
                                    }
                                    _ => (),
                                }
                            }
                            'i' => {
                                let d1 = init_itra.next().unwrap();
                                match d1 {
                                    'n' => {
                                        let d2 = init_itra.next().unwrap();
                                        match d2 {
                                            's' => {
                                                //'inset'
                                                let mut v2 = init_itra.next().unwrap();
                                                let mut v3 = String::new();
                                                loop {
                                                    v3 = v3 + &String::from(v2);
                                                    match init_itra.next() {
                                                        Some(next_char_value) => {
                                                            v2 = next_char_value
                                                        }
                                                        None => break,
                                                    }
                                                }
                                                if variant == "" {
                                                    let v4 = v.to_string()
                                                        + &d1.to_string()
                                                        + &d2.to_string()
                                                        + &v3;
                                                    positioning.push((v4, 2));
                                                } else {
                                                    let v4 = variant.to_string()
                                                        + &v.to_string()
                                                        + &d1.to_string()
                                                        + &d2.to_string()
                                                        + &v3;
                                                    positioning.push((v4, 2));
                                                }
                                            }
                                            'l' => {
                                                //'inline'
                                                let mut v2 = init_itra.next().unwrap();
                                                let mut v3 = String::new();
                                                loop {
                                                    v3 = v3 + &String::from(v2);
                                                    match init_itra.next() {
                                                        Some(next_char_value) => {
                                                            v2 = next_char_value
                                                        }
                                                        None => break,
                                                    }
                                                }
                                                if variant == "" {
                                                    let v4 = v.to_string()
                                                        + &d1.to_string()
                                                        + &d2.to_string()
                                                        + &v3;
                                                    box_model.push((v4, 1));
                                                } else {
                                                    let v4 = variant.to_string()
                                                        + &v.to_string()
                                                        + &d1.to_string()
                                                        + &d2.to_string()
                                                        + &v3;
                                                    box_model.push((v4, 1));
                                                }
                                            }
                                            'd' => {
                                                //'indent'
                                                let mut v2 = init_itra.next().unwrap();
                                                let mut v3 = String::new();
                                                loop {
                                                    v3 = v3 + &String::from(v2);
                                                    match init_itra.next() {
                                                        Some(next_char_value) => {
                                                            v2 = next_char_value
                                                        }
                                                        None => break,
                                                    }
                                                }
                                                if variant == "" {
                                                    let v4 = v.to_string()
                                                        + &d1.to_string()
                                                        + &d2.to_string()
                                                        + &v3;
                                                    typography.push((v4, 13));
                                                } else {
                                                    let v4 = variant.to_string()
                                                        + &v.to_string()
                                                        + &d1.to_string()
                                                        + &d2.to_string()
                                                        + &v3;
                                                    typography.push((v4, 13));
                                                }
                                            }
                                            'v' => {
                                                //'invert'
                                                let mut v2 = init_itra.next().unwrap();
                                                let mut v3 = String::new();
                                                loop {
                                                    v3 = v3 + &String::from(v2);
                                                    match init_itra.next() {
                                                        Some(next_char_value) => {
                                                            v2 = next_char_value
                                                        }
                                                        None => break,
                                                    }
                                                }
                                                if variant == "" {
                                                    let v4 = v.to_string()
                                                        + &d1.to_string()
                                                        + &d2.to_string()
                                                        + &v3;
                                                    effects.push((v4, 8));
                                                } else {
                                                    let v4 = variant.to_string()
                                                        + &v.to_string()
                                                        + &d1.to_string()
                                                        + &d2.to_string()
                                                        + &v3;
                                                    effects.push((v4, 8));
                                                }
                                            }
                                            _ => (),
                                        }
                                    }
                                    's' => {
                                        let d2 = init_itra.next().unwrap();
                                        match d2 {
                                            'o' => {
                                                let d3 = init_itra.next().unwrap();
                                                match d3 {
                                                    'l' => {
                                                        let d4 = init_itra.next().unwrap();
                                                        match d4 {
                                                            'a' => {
                                                                let d5 = init_itra.next().unwrap();
                                                                match d5 {
                                                                    't' => {
                                                                        let d6 = init_itra
                                                                            .next()
                                                                            .unwrap();
                                                                        match d6 {
                                                                            'e' => {
                                                                                //'isolate'
                                                                                let mut v2 =
                                                                                    init_itra
                                                                                        .next()
                                                                                        .unwrap();
                                                                                let mut v3 =
                                                                                    String::new();
                                                                                loop {
                                                                                    v3 = v3 + &String::from(v2);
                                                                                    match init_itra.next() {
    																	        Some(next_char_value) => v2 = next_char_value,
    																	        None => break,
    																	    }
                                                                                }
                                                                                if variant == "" {
                                                                                    let v4 = v.to_string() + &d1.to_string()+ &d2.to_string()+ &d3.to_string()+ &d4.to_string()+ &d5.to_string()+ &d6.to_string() + &v3;
                                                                                    box_model.push(
                                                                                        (v4, 0),
                                                                                    );
                                                                                } else {
                                                                                    let v4 = variant.to_string() + &v.to_string() + &d1.to_string()+ &d2.to_string()+ &d3.to_string()+ &d4.to_string()+ &d5.to_string()+ &d6.to_string() + &v3;
                                                                                    box_model.push(
                                                                                        (v4, 0),
                                                                                    );
                                                                                }
                                                                            }
                                                                            'i' => {
                                                                                //'isolation-auto'
                                                                                let mut v2 =
                                                                                    init_itra
                                                                                        .next()
                                                                                        .unwrap();
                                                                                let mut v3 =
                                                                                    String::new();
                                                                                loop {
                                                                                    v3 = v3 + &String::from(v2);
                                                                                    match init_itra.next() {
    																	        Some(next_char_value) => v2 = next_char_value,
    																	        None => break,
    																	    }
                                                                                }
                                                                                if variant == "" {
                                                                                    let v4 = v.to_string() + &d1.to_string()+ &d2.to_string()+ &d3.to_string()+ &d4.to_string()+ &d5.to_string()+ &d6.to_string() + &v3;
                                                                                    box_model.push(
                                                                                        (v4, 0),
                                                                                    );
                                                                                } else {
                                                                                    let v4 = variant.to_string() + &v.to_string() + &d1.to_string()+ &d2.to_string()+ &d3.to_string()+ &d4.to_string()+ &d5.to_string()+ &d6.to_string() + &v3;
                                                                                    box_model.push(
                                                                                        (v4, 0),
                                                                                    );
                                                                                }
                                                                            }
                                                                            _ => (),
                                                                        }
                                                                    }
                                                                    _ => (),
                                                                }
                                                            }
                                                            _ => (),
                                                        }
                                                    }
                                                    _ => (),
                                                }
                                            }
                                            _ => (),
                                        }
                                    }
                                    't' => {
                                        let d2 = init_itra.next().unwrap();
                                        match d2 {
                                            'e' => {
                                                //'items'
                                                let mut v2 = init_itra.next().unwrap();
                                                let mut v3 = String::new();
                                                loop {
                                                    v3 = v3 + &String::from(v2);
                                                    match init_itra.next() {
                                                        Some(next_char_value) => {
                                                            v2 = next_char_value
                                                        }
                                                        None => break,
                                                    }
                                                }
                                                if variant == "" {
                                                    let v4 = v.to_string()
                                                        + &d1.to_string()
                                                        + &d2.to_string()
                                                        + &v3;
                                                    alignment.push((v4, 1));
                                                } else {
                                                    let v4 = variant.to_string()
                                                        + &v.to_string()
                                                        + &d1.to_string()
                                                        + &d2.to_string()
                                                        + &v3;
                                                    alignment.push((v4, 1));
                                                }
                                            }
                                            'a' => {
                                                //'italic'
                                                let mut v2 = init_itra.next().unwrap();
                                                let mut v3 = String::new();
                                                loop {
                                                    v3 = v3 + &String::from(v2);
                                                    match init_itra.next() {
                                                        Some(next_char_value) => {
                                                            v2 = next_char_value
                                                        }
                                                        None => break,
                                                    }
                                                }
                                                if variant == "" {
                                                    let v4 = v.to_string()
                                                        + &d1.to_string()
                                                        + &d2.to_string()
                                                        + &v3;
                                                    typography.push((v4, 5));
                                                } else {
                                                    let v4 = variant.to_string()
                                                        + &v.to_string()
                                                        + &d1.to_string()
                                                        + &d2.to_string()
                                                        + &v3;
                                                    typography.push((v4, 5));
                                                }
                                            }
                                            _ => (),
                                        }
                                    }
                                    _ => (),
                                }
                            }
                            'j' => {
                                //'justify'
                                let mut v2 = init_itra.next().unwrap();
                                let mut v3 = String::new();
                                loop {
                                    v3 = v3 + &String::from(v2);
                                    match init_itra.next() {
                                        Some(next_char_value) => v2 = next_char_value,
                                        None => break,
                                    }
                                }
                                if variant == "" {
                                    let v4 = v.to_string() + &v3;
                                    alignment.push((v4, 2));
                                } else {
                                    let v4 = variant.to_string() + &v.to_string() + &v3;
                                    alignment.push((v4, 2));
                                }
                            }
                            'l' => {
                                let d1 = init_itra.next().unwrap();
                                match d1 {
                                    'e' => {
                                        let d2 = init_itra.next().unwrap();
                                        match d2 {
                                            'f' => {
                                                //'left'
                                                let mut v2 = init_itra.next().unwrap();
                                                let mut v3 = String::new();
                                                loop {
                                                    v3 = v3 + &String::from(v2);
                                                    match init_itra.next() {
                                                        Some(next_char_value) => {
                                                            v2 = next_char_value
                                                        }
                                                        None => break,
                                                    }
                                                }
                                                if variant == "" {
                                                    let v4 = v.to_string()
                                                        + &d1.to_string()
                                                        + &d2.to_string()
                                                        + &v3;
                                                    positioning.push((v4, 6));
                                                } else {
                                                    let v4 = variant.to_string()
                                                        + &v.to_string()
                                                        + &d1.to_string()
                                                        + &d2.to_string()
                                                        + &v3;
                                                    positioning.push((v4, 6));
                                                }
                                            }
                                            'a' => {
                                                //'leading'
                                                let mut v2 = init_itra.next().unwrap();
                                                let mut v3 = String::new();
                                                loop {
                                                    v3 = v3 + &String::from(v2);
                                                    match init_itra.next() {
                                                        Some(next_char_value) => {
                                                            v2 = next_char_value
                                                        }
                                                        None => break,
                                                    }
                                                }
                                                if variant == "" {
                                                    let v4 = v.to_string()
                                                        + &d1.to_string()
                                                        + &d2.to_string()
                                                        + &v3;
                                                    typography.push((v4, 8));
                                                } else {
                                                    let v4 = variant.to_string()
                                                        + &v.to_string()
                                                        + &d1.to_string()
                                                        + &d2.to_string()
                                                        + &v3;
                                                    typography.push((v4, 8));
                                                }
                                            }
                                            _ => (),
                                        }
                                    }
                                    'i' => {
                                        let d2 = init_itra.next().unwrap();
                                        match d2 {
                                            's' => {
                                                let d3 = init_itra.next().unwrap();
                                                match d3 {
                                                    't' => {
                                                        let d4 = init_itra.next().unwrap();
                                                        match d4 {
                                                            '-' => {
                                                                let d5 = init_itra.next().unwrap();
                                                                match d5 {
                                                                    'i' => {
                                                                        let d6 = init_itra
                                                                            .next()
                                                                            .unwrap();
                                                                        match d6 {
                                                                            't' => {
                                                                                //'list-item'
                                                                                let mut v2 =
                                                                                    init_itra
                                                                                        .next()
                                                                                        .unwrap();
                                                                                let mut v3 =
                                                                                    String::new();
                                                                                loop {
                                                                                    v3 = v3 + &String::from(v2);
                                                                                    match init_itra.next() {
    																	        Some(next_char_value) => v2 = next_char_value,
    																	        None => break,
    																	    }
                                                                                }
                                                                                if variant == "" {
                                                                                    let v4 = v.to_string() + &d1.to_string()+ &d2.to_string()+ &d3.to_string()+ &d4.to_string()+ &d5.to_string()+ &d6.to_string() + &v3;
                                                                                    box_model.push(
                                                                                        (v4, 1),
                                                                                    );
                                                                                } else {
                                                                                    let v4 = variant.to_string() + &v.to_string() + &d1.to_string()+ &d2.to_string()+ &d3.to_string()+ &d4.to_string()+ &d5.to_string()+ &d6.to_string() + &v3;
                                                                                    box_model.push(
                                                                                        (v4, 1),
                                                                                    );
                                                                                }
                                                                            }
                                                                            'n' => {
                                                                                //'list-inside'
                                                                                let mut v2 =
                                                                                    init_itra
                                                                                        .next()
                                                                                        .unwrap();
                                                                                let mut v3 =
                                                                                    String::new();
                                                                                loop {
                                                                                    v3 = v3 + &String::from(v2);
                                                                                    match init_itra.next() {
    																	        Some(next_char_value) => v2 = next_char_value,
    																	        None => break,
    																	    }
                                                                                }
                                                                                if variant == "" {
                                                                                    let v4 = v.to_string() + &d1.to_string()+ &d2.to_string()+ &d3.to_string()+ &d4.to_string()+ &d5.to_string()+ &d6.to_string() + &v3;
                                                                                    typography
                                                                                        .push((
                                                                                            v4, 3,
                                                                                        ));
                                                                                } else {
                                                                                    let v4 = variant.to_string() + &v.to_string() + &d1.to_string()+ &d2.to_string()+ &d3.to_string()+ &d4.to_string()+ &d5.to_string()+ &d6.to_string() + &v3;
                                                                                    typography
                                                                                        .push((
                                                                                            v4, 3,
                                                                                        ));
                                                                                }
                                                                            }
                                                                            _ => (),
                                                                        }
                                                                    }
                                                                    'n' => {
                                                                        //'list-none'
                                                                        let mut v2 = init_itra
                                                                            .next()
                                                                            .unwrap();
                                                                        let mut v3 = String::new();
                                                                        loop {
                                                                            v3 = v3
                                                                                + &String::from(v2);
                                                                            match init_itra.next() {
                                                                                Some(
                                                                                    next_char_value,
                                                                                ) => v2 =
                                                                                    next_char_value,
                                                                                None => break,
                                                                            }
                                                                        }
                                                                        if variant == "" {
                                                                            let v4 = v.to_string()
                                                                                + &d1.to_string()
                                                                                + &d2.to_string()
                                                                                + &d3.to_string()
                                                                                + &d4.to_string()
                                                                                + &d5.to_string()
                                                                                + &v3;
                                                                            typography
                                                                                .push((v4, 3));
                                                                        } else {
                                                                            let v4 = variant
                                                                                .to_string()
                                                                                + &v.to_string()
                                                                                + &d1.to_string()
                                                                                + &d2.to_string()
                                                                                + &d3.to_string()
                                                                                + &d4.to_string()
                                                                                + &d5.to_string()
                                                                                + &v3;
                                                                            typography
                                                                                .push((v4, 3));
                                                                        }
                                                                    }
                                                                    'd' => {
                                                                        let d6 = init_itra
                                                                            .next()
                                                                            .unwrap();
                                                                        match d6 {
                                                                            'i' => {
                                                                                //'list-disc'
                                                                                let mut v2 =
                                                                                    init_itra
                                                                                        .next()
                                                                                        .unwrap();
                                                                                let mut v3 =
                                                                                    String::new();
                                                                                loop {
                                                                                    v3 = v3 + &String::from(v2);
                                                                                    match init_itra.next() {
    																	        Some(next_char_value) => v2 = next_char_value,
    																	        None => break,
    																	    }
                                                                                }
                                                                                if variant == "" {
                                                                                    let v4 = v.to_string() + &d1.to_string()+ &d2.to_string()+ &d3.to_string()+ &d4.to_string()+ &d5.to_string()+ &d6.to_string() + &v3;
                                                                                    typography
                                                                                        .push((
                                                                                            v4, 3,
                                                                                        ));
                                                                                } else {
                                                                                    let v4 = variant.to_string() + &v.to_string() + &d1.to_string()+ &d2.to_string()+ &d3.to_string()+ &d4.to_string()+ &d5.to_string()+ &d6.to_string() + &v3;
                                                                                    typography
                                                                                        .push((
                                                                                            v4, 3,
                                                                                        ));
                                                                                }
                                                                            }
                                                                            'e' => {
                                                                                //'list-decimal'
                                                                                let mut v2 =
                                                                                    init_itra
                                                                                        .next()
                                                                                        .unwrap();
                                                                                let mut v3 =
                                                                                    String::new();
                                                                                loop {
                                                                                    v3 = v3 + &String::from(v2);
                                                                                    match init_itra.next() {
    																	        Some(next_char_value) => v2 = next_char_value,
    																	        None => break,
    																	    }
                                                                                }
                                                                                if variant == "" {
                                                                                    let v4 = v.to_string() + &d1.to_string()+ &d2.to_string()+ &d3.to_string()+ &d4.to_string()+ &d5.to_string()+ &d6.to_string() + &v3;
                                                                                    typography
                                                                                        .push((
                                                                                            v4, 3,
                                                                                        ));
                                                                                } else {
                                                                                    let v4 = variant.to_string() + &v.to_string() + &d1.to_string()+ &d2.to_string()+ &d3.to_string()+ &d4.to_string()+ &d5.to_string()+ &d6.to_string() + &v3;
                                                                                    typography
                                                                                        .push((
                                                                                            v4, 3,
                                                                                        ));
                                                                                }
                                                                            }
                                                                            _ => (),
                                                                        }
                                                                    }
                                                                    'o' => {
                                                                        //'list-outside'
                                                                        let mut v2 = init_itra
                                                                            .next()
                                                                            .unwrap();
                                                                        let mut v3 = String::new();
                                                                        loop {
                                                                            v3 = v3
                                                                                + &String::from(v2);
                                                                            match init_itra.next() {
                                                                                Some(
                                                                                    next_char_value,
                                                                                ) => v2 =
                                                                                    next_char_value,
                                                                                None => break,
                                                                            }
                                                                        }
                                                                        if variant == "" {
                                                                            let v4 = v.to_string()
                                                                                + &d1.to_string()
                                                                                + &d2.to_string()
                                                                                + &d3.to_string()
                                                                                + &d4.to_string()
                                                                                + &d5.to_string()
                                                                                + &v3;
                                                                            typography
                                                                                .push((v4, 3));
                                                                        } else {
                                                                            let v4 = variant
                                                                                .to_string()
                                                                                + &v.to_string()
                                                                                + &d1.to_string()
                                                                                + &d2.to_string()
                                                                                + &d3.to_string()
                                                                                + &d4.to_string()
                                                                                + &d5.to_string()
                                                                                + &v3;
                                                                            typography
                                                                                .push((v4, 3));
                                                                        }
                                                                    }
                                                                    _ => (),
                                                                }
                                                            }
                                                            _ => (),
                                                        }
                                                    }
                                                    _ => (),
                                                }
                                            }
                                            'n' => {
                                                //'lining-nums'
                                                let mut v2 = init_itra.next().unwrap();
                                                let mut v3 = String::new();
                                                loop {
                                                    v3 = v3 + &String::from(v2);
                                                    match init_itra.next() {
                                                        Some(next_char_value) => {
                                                            v2 = next_char_value
                                                        }
                                                        None => break,
                                                    }
                                                }
                                                if variant == "" {
                                                    let v4 = v.to_string()
                                                        + &d1.to_string()
                                                        + &d2.to_string()
                                                        + &v3;
                                                    typography.push((v4, 9));
                                                } else {
                                                    let v4 = variant.to_string()
                                                        + &v.to_string()
                                                        + &d1.to_string()
                                                        + &d2.to_string()
                                                        + &v3;
                                                    typography.push((v4, 9));
                                                }
                                            }
                                            _ => (),
                                        }
                                    }
                                    'o' => {
                                        //'lowercase'
                                        let mut v2 = init_itra.next().unwrap();
                                        let mut v3 = String::new();
                                        loop {
                                            v3 = v3 + &String::from(v2);
                                            match init_itra.next() {
                                                Some(next_char_value) => v2 = next_char_value,
                                                None => break,
                                            }
                                        }
                                        if variant == "" {
                                            let v4 = v.to_string() + &d1.to_string() + &v3;
                                            typography.push((v4, 11));
                                        } else {
                                            let v4 = variant.to_string()
                                                + &v.to_string()
                                                + &d1.to_string()
                                                + &v3;
                                            typography.push((v4, 11));
                                        }
                                    }
                                    _ => (),
                                }
                            }
                            'm' => {
                                let d1 = init_itra.next().unwrap();
                                match d1 {
                                    'a' => {
                                        let d2 = init_itra.next().unwrap();
                                        match d2 {
                                            'x' => {
                                                let d3 = init_itra.next().unwrap();
                                                match d3 {
                                                    '-' => {
                                                        let d4 = init_itra.next().unwrap();
                                                        match d4 {
                                                            'w' => {
                                                                //'max-w'
                                                                let mut v2 =
                                                                    init_itra.next().unwrap();
                                                                let mut v3 = String::new();
                                                                loop {
                                                                    v3 = v3 + &String::from(v2);
                                                                    match init_itra.next() {
                                                                        Some(next_char_value) => {
                                                                            v2 = next_char_value
                                                                        }
                                                                        None => break,
                                                                    }
                                                                }
                                                                if variant == "" {
                                                                    let v4 = v.to_string()
                                                                        + &d1.to_string()
                                                                        + &d2.to_string()
                                                                        + &d3.to_string()
                                                                        + &d4.to_string()
                                                                        + &v3;
                                                                    box_model.push((v4, 5));
                                                                } else {
                                                                    let v4 = variant.to_string()
                                                                        + &v.to_string()
                                                                        + &d1.to_string()
                                                                        + &d2.to_string()
                                                                        + &d3.to_string()
                                                                        + &d4.to_string()
                                                                        + &v3;
                                                                    box_model.push((v4, 5));
                                                                }
                                                            }
                                                            'h' => {
                                                                //'max-h'
                                                                let mut v2 =
                                                                    init_itra.next().unwrap();
                                                                let mut v3 = String::new();
                                                                loop {
                                                                    v3 = v3 + &String::from(v2);
                                                                    match init_itra.next() {
                                                                        Some(next_char_value) => {
                                                                            v2 = next_char_value
                                                                        }
                                                                        None => break,
                                                                    }
                                                                }
                                                                if variant == "" {
                                                                    let v4 = v.to_string()
                                                                        + &d1.to_string()
                                                                        + &d2.to_string()
                                                                        + &d3.to_string()
                                                                        + &d4.to_string()
                                                                        + &v3;
                                                                    box_model.push((v4, 8));
                                                                } else {
                                                                    let v4 = variant.to_string()
                                                                        + &v.to_string()
                                                                        + &d1.to_string()
                                                                        + &d2.to_string()
                                                                        + &d3.to_string()
                                                                        + &d4.to_string()
                                                                        + &v3;
                                                                    box_model.push((v4, 8));
                                                                }
                                                            }
                                                            _ => (),
                                                        }
                                                    }
                                                    _ => (),
                                                }
                                            }
                                            _ => (),
                                        }
                                    }
                                    'i' => {
                                        let d2 = init_itra.next().unwrap();
                                        match d2 {
                                            'n' => {
                                                let d3 = init_itra.next().unwrap();
                                                match d3 {
                                                    '-' => {
                                                        let d4 = init_itra.next().unwrap();
                                                        match d4 {
                                                            'w' => {
                                                                //'min-w'
                                                                let mut v2 =
                                                                    init_itra.next().unwrap();
                                                                let mut v3 = String::new();
                                                                loop {
                                                                    v3 = v3 + &String::from(v2);
                                                                    match init_itra.next() {
                                                                        Some(next_char_value) => {
                                                                            v2 = next_char_value
                                                                        }
                                                                        None => break,
                                                                    }
                                                                }
                                                                if variant == "" {
                                                                    let v4 = v.to_string()
                                                                        + &d1.to_string()
                                                                        + &d2.to_string()
                                                                        + &d3.to_string()
                                                                        + &d4.to_string()
                                                                        + &v3;
                                                                    box_model.push((v4, 6));
                                                                } else {
                                                                    let v4 = variant.to_string()
                                                                        + &v.to_string()
                                                                        + &d1.to_string()
                                                                        + &d2.to_string()
                                                                        + &d3.to_string()
                                                                        + &d4.to_string()
                                                                        + &v3;
                                                                    box_model.push((v4, 6));
                                                                }
                                                            }
                                                            'h' => {
                                                                //'min-h'
                                                                let mut v2 =
                                                                    init_itra.next().unwrap();
                                                                let mut v3 = String::new();
                                                                loop {
                                                                    v3 = v3 + &String::from(v2);
                                                                    match init_itra.next() {
                                                                        Some(next_char_value) => {
                                                                            v2 = next_char_value
                                                                        }
                                                                        None => break,
                                                                    }
                                                                }
                                                                if variant == "" {
                                                                    let v4 = v.to_string()
                                                                        + &d1.to_string()
                                                                        + &d2.to_string()
                                                                        + &d3.to_string()
                                                                        + &d4.to_string()
                                                                        + &v3;
                                                                    box_model.push((v4, 9));
                                                                } else {
                                                                    let v4 = variant.to_string()
                                                                        + &v.to_string()
                                                                        + &d1.to_string()
                                                                        + &d2.to_string()
                                                                        + &d3.to_string()
                                                                        + &d4.to_string()
                                                                        + &v3;
                                                                    box_model.push((v4, 9));
                                                                }
                                                            }
                                                            _ => (),
                                                        }
                                                    }
                                                    _ => (),
                                                }
                                            }
                                            'x' => {
                                                //'mix-blend'
                                                let mut v2 = init_itra.next().unwrap();
                                                let mut v3 = String::new();
                                                loop {
                                                    v3 = v3 + &String::from(v2);
                                                    match init_itra.next() {
                                                        Some(next_char_value) => {
                                                            v2 = next_char_value
                                                        }
                                                        None => break,
                                                    }
                                                }
                                                if variant == "" {
                                                    let v4 = v.to_string()
                                                        + &d1.to_string()
                                                        + &d2.to_string()
                                                        + &v3;
                                                    effects.push((v4, 2));
                                                } else {
                                                    let v4 = variant.to_string()
                                                        + &v.to_string()
                                                        + &d1.to_string()
                                                        + &d2.to_string()
                                                        + &v3;
                                                    effects.push((v4, 2));
                                                }
                                            }
                                            _ => (),
                                        }
                                    }
                                    '-' => {
                                        //'m-'
                                        let mut v2 = init_itra.next().unwrap();
                                        let mut v3 = String::new();
                                        loop {
                                            v3 = v3 + &String::from(v2);
                                            match init_itra.next() {
                                                Some(next_char_value) => v2 = next_char_value,
                                                None => break,
                                            }
                                        }
                                        if variant == "" {
                                            let v4 = v.to_string() + &d1.to_string() + &v3;
                                            box_model.push((v4, 41));
                                        } else {
                                            let v4 = variant.to_string()
                                                + &v.to_string()
                                                + &d1.to_string()
                                                + &v3;
                                            box_model.push((v4, 41));
                                        }
                                    }
                                    'x' => {
                                        //'mx'
                                        let mut v2 = init_itra.next().unwrap();
                                        let mut v3 = String::new();
                                        loop {
                                            v3 = v3 + &String::from(v2);
                                            match init_itra.next() {
                                                Some(next_char_value) => v2 = next_char_value,
                                                None => break,
                                            }
                                        }
                                        if variant == "" {
                                            let v4 = v.to_string() + &d1.to_string() + &v3;
                                            box_model.push((v4, 42));
                                        } else {
                                            let v4 = variant.to_string()
                                                + &v.to_string()
                                                + &d1.to_string()
                                                + &v3;
                                            box_model.push((v4, 42));
                                        }
                                    }
                                    'y' => {
                                        //'my'
                                        let mut v2 = init_itra.next().unwrap();
                                        let mut v3 = String::new();
                                        loop {
                                            v3 = v3 + &String::from(v2);
                                            match init_itra.next() {
                                                Some(next_char_value) => v2 = next_char_value,
                                                None => break,
                                            }
                                        }
                                        if variant == "" {
                                            let v4 = v.to_string() + &d1.to_string() + &v3;
                                            box_model.push((v4, 43));
                                        } else {
                                            let v4 = variant.to_string()
                                                + &v.to_string()
                                                + &d1.to_string()
                                                + &v3;
                                            box_model.push((v4, 43));
                                        }
                                    }
                                    't' => {
                                        //'mt'
                                        let mut v2 = init_itra.next().unwrap();
                                        let mut v3 = String::new();
                                        loop {
                                            v3 = v3 + &String::from(v2);
                                            match init_itra.next() {
                                                Some(next_char_value) => v2 = next_char_value,
                                                None => break,
                                            }
                                        }
                                        if variant == "" {
                                            let v4 = v.to_string() + &d1.to_string() + &v3;
                                            box_model.push((v4, 44));
                                        } else {
                                            let v4 = variant.to_string()
                                                + &v.to_string()
                                                + &d1.to_string()
                                                + &v3;
                                            box_model.push((v4, 44));
                                        }
                                    }
                                    'l' => {
                                        //'ml'
                                        let mut v2 = init_itra.next().unwrap();
                                        let mut v3 = String::new();
                                        loop {
                                            v3 = v3 + &String::from(v2);
                                            match init_itra.next() {
                                                Some(next_char_value) => v2 = next_char_value,
                                                None => break,
                                            }
                                        }
                                        if variant == "" {
                                            let v4 = v.to_string() + &d1.to_string() + &v3;
                                            box_model.push((v4, 45));
                                        } else {
                                            let v4 = variant.to_string()
                                                + &v.to_string()
                                                + &d1.to_string()
                                                + &v3;
                                            box_model.push((v4, 45));
                                        }
                                    }
                                    'r' => {
                                        //'mr'
                                        let mut v2 = init_itra.next().unwrap();
                                        let mut v3 = String::new();
                                        loop {
                                            v3 = v3 + &String::from(v2);
                                            match init_itra.next() {
                                                Some(next_char_value) => v2 = next_char_value,
                                                None => break,
                                            }
                                        }
                                        if variant == "" {
                                            let v4 = v.to_string() + &d1.to_string() + &v3;
                                            box_model.push((v4, 46));
                                        } else {
                                            let v4 = variant.to_string()
                                                + &v.to_string()
                                                + &d1.to_string()
                                                + &v3;
                                            box_model.push((v4, 46));
                                        }
                                    }
                                    'b' => {
                                        //'mb'
                                        let mut v2 = init_itra.next().unwrap();
                                        let mut v3 = String::new();
                                        loop {
                                            v3 = v3 + &String::from(v2);
                                            match init_itra.next() {
                                                Some(next_char_value) => v2 = next_char_value,
                                                None => break,
                                            }
                                        }
                                        if variant == "" {
                                            let v4 = v.to_string() + &d1.to_string() + &v3;
                                            box_model.push((v4, 47));
                                        } else {
                                            let v4 = variant.to_string()
                                                + &v.to_string()
                                                + &d1.to_string()
                                                + &v3;
                                            box_model.push((v4, 47));
                                        }
                                    }
                                    _ => (),
                                }
                            }
                            'n' => {
                                let d1 = init_itra.next().unwrap();
                                match d1 {
                                    'o' => {
                                        let d2 = init_itra.next().unwrap();
                                        match d2 {
                                            't' => {
                                                let d3 = init_itra.next().unwrap();
                                                match d3 {
                                                    '-' => {
                                                        let d4 = init_itra.next().unwrap();
                                                        match d4 {
                                                            'i' => {
                                                                //'not-italic'
                                                                let mut v2 =
                                                                    init_itra.next().unwrap();
                                                                let mut v3 = String::new();
                                                                loop {
                                                                    v3 = v3 + &String::from(v2);
                                                                    match init_itra.next() {
                                                                        Some(next_char_value) => {
                                                                            v2 = next_char_value
                                                                        }
                                                                        None => break,
                                                                    }
                                                                }
                                                                if variant == "" {
                                                                    let v4 = v.to_string()
                                                                        + &d1.to_string()
                                                                        + &d2.to_string()
                                                                        + &d3.to_string()
                                                                        + &d4.to_string()
                                                                        + &v3;
                                                                    typography.push((v4, 6));
                                                                } else {
                                                                    let v4 = variant.to_string()
                                                                        + &v.to_string()
                                                                        + &d1.to_string()
                                                                        + &d2.to_string()
                                                                        + &d3.to_string()
                                                                        + &d4.to_string()
                                                                        + &v3;
                                                                    typography.push((v4, 6));
                                                                }
                                                            }
                                                            's' => {
                                                                //'not-sr-only'
                                                                let mut v2 =
                                                                    init_itra.next().unwrap();
                                                                let mut v3 = String::new();
                                                                loop {
                                                                    v3 = v3 + &String::from(v2);
                                                                    match init_itra.next() {
                                                                        Some(next_char_value) => {
                                                                            v2 = next_char_value
                                                                        }
                                                                        None => break,
                                                                    }
                                                                }
                                                                if variant == "" {
                                                                    let v4 = v.to_string()
                                                                        + &d1.to_string()
                                                                        + &d2.to_string()
                                                                        + &d3.to_string()
                                                                        + &d4.to_string()
                                                                        + &v3;
                                                                    misc.push((v4, 1));
                                                                } else {
                                                                    let v4 = variant.to_string()
                                                                        + &v.to_string()
                                                                        + &d1.to_string()
                                                                        + &d2.to_string()
                                                                        + &d3.to_string()
                                                                        + &d4.to_string()
                                                                        + &v3;
                                                                    misc.push((v4, 1));
                                                                }
                                                            }
                                                            _ => (),
                                                        }
                                                    }
                                                    _ => (),
                                                }
                                            }
                                            'r' => {
                                                let d3 = init_itra.next().unwrap();
                                                match d3 {
                                                    'm' => {
                                                        let d4 = init_itra.next().unwrap();
                                                        match d4 {
                                                            'a' => {
                                                                let d5 = init_itra.next().unwrap();
                                                                match d5 {
                                                                    'l' => {
                                                                        let d6 = init_itra
                                                                            .next()
                                                                            .unwrap();
                                                                        match d6 {
                                                                            '-' => {
                                                                                let d7 = init_itra
                                                                                    .next()
                                                                                    .unwrap();
                                                                                match d7 {
                                                                                    'n' => {
                                                                                        //'normal-nums'
                                                                                        let mut v2 = init_itra.next().unwrap();
                                                                                        let mut v3 = String::new();
                                                                                        loop {
                                                                                            v3 = v3 + &String::from(v2);
                                                                                            match init_itra.next() {
    																			        Some(next_char_value) => v2 = next_char_value,
    																			        None => break,
    																			    }
                                                                                        }
                                                                                        if variant
                                                                                            == ""
                                                                                        {
                                                                                            let v4 = v.to_string() + &d1.to_string()+ &d2.to_string()+ &d3.to_string()+ &d4.to_string()+ &d5.to_string()+ &d6.to_string()+ &d7.to_string() + &v3;
                                                                                            typography.push((
																						v4,
    																			  		9,
    																				));
                                                                                        } else {
                                                                                            let v4 = variant.to_string() + &v.to_string() + &d1.to_string()+ &d2.to_string()+ &d3.to_string()+ &d4.to_string()+ &d5.to_string()+ &d6.to_string()+ &d7.to_string() + &v3;
                                                                                            typography.push((
    																			  v4,
    																			  9,
    																			));
                                                                                        }
                                                                                    }
                                                                                    'c' => {
                                                                                        //'normal-case'
                                                                                        let mut v2 = init_itra.next().unwrap();
                                                                                        let mut v3 = String::new();
                                                                                        loop {
                                                                                            v3 = v3 + &String::from(v2);
                                                                                            match init_itra.next() {
    																			        Some(next_char_value) => v2 = next_char_value,
    																			        None => break,
    																			    }
                                                                                        }
                                                                                        if variant
                                                                                            == ""
                                                                                        {
                                                                                            let v4 = v.to_string() + &d1.to_string()+ &d2.to_string()+ &d3.to_string()+ &d4.to_string()+ &d5.to_string()+ &d6.to_string()+ &d7.to_string() + &v3;
                                                                                            typography.push((
																						v4,
    																			  		11,
    																				));
                                                                                        } else {
                                                                                            let v4 = variant.to_string() + &v.to_string() + &d1.to_string()+ &d2.to_string()+ &d3.to_string()+ &d4.to_string()+ &d5.to_string()+ &d6.to_string()+ &d7.to_string() + &v3;
                                                                                            typography.push((
    																			  v4,
    																			  11,
    																			));
                                                                                        }
                                                                                    }
                                                                                    _ => (),
                                                                                }
                                                                            }
                                                                            _ => (),
                                                                        }
                                                                    }
                                                                    _ => (),
                                                                }
                                                            }
                                                            _ => (),
                                                        }
                                                    }
                                                    _ => (),
                                                }
                                            }
                                            '-' => {
                                                //'no-underline'
                                                let mut v2 = init_itra.next().unwrap();
                                                let mut v3 = String::new();
                                                loop {
                                                    v3 = v3 + &String::from(v2);
                                                    match init_itra.next() {
                                                        Some(next_char_value) => {
                                                            v2 = next_char_value
                                                        }
                                                        None => break,
                                                    }
                                                }
                                                if variant == "" {
                                                    let v4 = v.to_string()
                                                        + &d1.to_string()
                                                        + &d2.to_string()
                                                        + &v3;
                                                    typography.push((v4, 10));
                                                } else {
                                                    let v4 = variant.to_string()
                                                        + &v.to_string()
                                                        + &d1.to_string()
                                                        + &d2.to_string()
                                                        + &v3;
                                                    typography.push((v4, 10));
                                                }
                                            }
                                            _ => (),
                                        }
                                    }
                                    _ => (),
                                }
                            }
                            'o' => {
                                let d1 = init_itra.next().unwrap();
                                match d1 {
                                    'b' => {
                                        //'object'
                                        let mut v2 = init_itra.next().unwrap();
                                        let mut v3 = String::new();
                                        loop {
                                            v3 = v3 + &String::from(v2);
                                            match init_itra.next() {
                                                Some(next_char_value) => v2 = next_char_value,
                                                None => break,
                                            }
                                        }
                                        if variant == "" {
                                            let v4 = v.to_string() + &d1.to_string() + &v3;
                                            positioning.push((v4, 7));
                                        } else {
                                            let v4 = variant.to_string()
                                                + &v.to_string()
                                                + &d1.to_string()
                                                + &v3;
                                            positioning.push((v4, 7));
                                        }
                                    }
                                    'v' => {
                                        let d2 = init_itra.next().unwrap();
                                        match d2 {
                                            'e' => {
                                                let d3 = init_itra.next().unwrap();
                                                match d3 {
                                                    'r' => {
                                                        let d4 = init_itra.next().unwrap();
                                                        match d4 {
                                                            'f' => {
                                                                //'overflow'
                                                                let mut v2 =
                                                                    init_itra.next().unwrap();
                                                                let mut v3 = String::new();
                                                                loop {
                                                                    v3 = v3 + &String::from(v2);
                                                                    match init_itra.next() {
                                                                        Some(next_char_value) => {
                                                                            v2 = next_char_value
                                                                        }
                                                                        None => break,
                                                                    }
                                                                }
                                                                if variant == "" {
                                                                    let v4 = v.to_string()
                                                                        + &d1.to_string()
                                                                        + &d2.to_string()
                                                                        + &d3.to_string()
                                                                        + &d4.to_string()
                                                                        + &v3;
                                                                    box_model.push((v4, 99));
                                                                } else {
                                                                    let v4 = variant.to_string()
                                                                        + &v.to_string()
                                                                        + &d1.to_string()
                                                                        + &d2.to_string()
                                                                        + &d3.to_string()
                                                                        + &d4.to_string()
                                                                        + &v3;
                                                                    box_model.push((v4, 99));
                                                                }
                                                            }
                                                            's' => {
                                                                //'overscroll'
                                                                let mut v2 =
                                                                    init_itra.next().unwrap();
                                                                let mut v3 = String::new();
                                                                loop {
                                                                    v3 = v3 + &String::from(v2);
                                                                    match init_itra.next() {
                                                                        Some(next_char_value) => {
                                                                            v2 = next_char_value
                                                                        }
                                                                        None => break,
                                                                    }
                                                                }
                                                                if variant == "" {
                                                                    let v4 = v.to_string()
                                                                        + &d1.to_string()
                                                                        + &d2.to_string()
                                                                        + &d3.to_string()
                                                                        + &d4.to_string()
                                                                        + &v3;
                                                                    box_model.push((v4, 99));
                                                                } else {
                                                                    let v4 = variant.to_string()
                                                                        + &v.to_string()
                                                                        + &d1.to_string()
                                                                        + &d2.to_string()
                                                                        + &d3.to_string()
                                                                        + &d4.to_string()
                                                                        + &v3;
                                                                    box_model.push((v4, 99));
                                                                }
                                                            }
                                                            'l' => {
                                                                //'overline'
                                                                let mut v2 =
                                                                    init_itra.next().unwrap();
                                                                let mut v3 = String::new();
                                                                loop {
                                                                    v3 = v3 + &String::from(v2);
                                                                    match init_itra.next() {
                                                                        Some(next_char_value) => {
                                                                            v2 = next_char_value
                                                                        }
                                                                        None => break,
                                                                    }
                                                                }
                                                                if variant == "" {
                                                                    let v4 = v.to_string()
                                                                        + &d1.to_string()
                                                                        + &d2.to_string()
                                                                        + &d3.to_string()
                                                                        + &d4.to_string()
                                                                        + &v3;
                                                                    typography.push((v4, 10));
                                                                } else {
                                                                    let v4 = variant.to_string()
                                                                        + &v.to_string()
                                                                        + &d1.to_string()
                                                                        + &d2.to_string()
                                                                        + &d3.to_string()
                                                                        + &d4.to_string()
                                                                        + &v3;
                                                                    typography.push((v4, 10));
                                                                }
                                                            }
                                                            _ => (),
                                                        }
                                                    }
                                                    _ => (),
                                                }
                                            }
                                            _ => (),
                                        }
                                    }
                                    'u' => {
                                        //'outline'
                                        let mut v2 = init_itra.next().unwrap();
                                        let mut v3 = String::new();
                                        loop {
                                            v3 = v3 + &String::from(v2);
                                            match init_itra.next() {
                                                Some(next_char_value) => v2 = next_char_value,
                                                None => break,
                                            }
                                        }
                                        if variant == "" {
                                            let v4 = v.to_string() + &d1.to_string() + &v3;
                                            box_model.push((v4, 22));
                                        } else {
                                            let v4 = variant.to_string()
                                                + &v.to_string()
                                                + &d1.to_string()
                                                + &v3;
                                            box_model.push((v4, 22));
                                        }
                                    }
                                    'r' => {
                                        let d2 = init_itra.next().unwrap();
                                        match d2 {
                                            'd' => {
                                                let d3 = init_itra.next().unwrap();
                                                match d3 {
                                                    'e' => {
                                                        //'order'
                                                        let mut v2 = init_itra.next().unwrap();
                                                        let mut v3 = String::new();
                                                        loop {
                                                            v3 = v3 + &String::from(v2);
                                                            match init_itra.next() {
                                                                Some(next_char_value) => {
                                                                    v2 = next_char_value
                                                                }
                                                                None => break,
                                                            }
                                                        }
                                                        if variant == "" {
                                                            let v4 = v.to_string()
                                                                + &d1.to_string()
                                                                + &d2.to_string()
                                                                + &d3.to_string()
                                                                + &v3;
                                                            flex_grid.push((v4, 4));
                                                        } else {
                                                            let v4 = variant.to_string()
                                                                + &v.to_string()
                                                                + &d1.to_string()
                                                                + &d2.to_string()
                                                                + &d3.to_string()
                                                                + &v3;
                                                            flex_grid.push((v4, 4));
                                                        }
                                                    }
                                                    'i' => {
                                                        //'ordinal'
                                                        let mut v2 = init_itra.next().unwrap();
                                                        let mut v3 = String::new();
                                                        loop {
                                                            v3 = v3 + &String::from(v2);
                                                            match init_itra.next() {
                                                                Some(next_char_value) => {
                                                                    v2 = next_char_value
                                                                }
                                                                None => break,
                                                            }
                                                        }
                                                        if variant == "" {
                                                            let v4 = v.to_string()
                                                                + &d1.to_string()
                                                                + &d2.to_string()
                                                                + &d3.to_string()
                                                                + &v3;
                                                            typography.push((v4, 9));
                                                        } else {
                                                            let v4 = variant.to_string()
                                                                + &v.to_string()
                                                                + &d1.to_string()
                                                                + &d2.to_string()
                                                                + &d3.to_string()
                                                                + &v3;
                                                            typography.push((v4, 9));
                                                        }
                                                    }
                                                    _ => (),
                                                }
                                            }
                                            'i' => {
                                                //'origin'
                                                let mut v2 = init_itra.next().unwrap();
                                                let mut v3 = String::new();
                                                loop {
                                                    v3 = v3 + &String::from(v2);
                                                    match init_itra.next() {
                                                        Some(next_char_value) => {
                                                            v2 = next_char_value
                                                        }
                                                        None => break,
                                                    }
                                                }
                                                if variant == "" {
                                                    let v4 = v.to_string()
                                                        + &d1.to_string()
                                                        + &d2.to_string()
                                                        + &v3;
                                                    transforms.push((v4, 1));
                                                } else {
                                                    let v4 = variant.to_string()
                                                        + &v.to_string()
                                                        + &d1.to_string()
                                                        + &d2.to_string()
                                                        + &v3;
                                                    transforms.push((v4, 1));
                                                }
                                            }
                                            _ => (),
                                        }
                                    }
                                    'l' => {
                                        //'oldstyle-nums'
                                        let mut v2 = init_itra.next().unwrap();
                                        let mut v3 = String::new();
                                        loop {
                                            v3 = v3 + &String::from(v2);
                                            match init_itra.next() {
                                                Some(next_char_value) => v2 = next_char_value,
                                                None => break,
                                            }
                                        }
                                        if variant == "" {
                                            let v4 = v.to_string() + &d1.to_string() + &v3;
                                            typography.push((v4, 9));
                                        } else {
                                            let v4 = variant.to_string()
                                                + &v.to_string()
                                                + &d1.to_string()
                                                + &v3;
                                            typography.push((v4, 9));
                                        }
                                    }
                                    'p' => {
                                        //'opacity'
                                        let mut v2 = init_itra.next().unwrap();
                                        let mut v3 = String::new();
                                        loop {
                                            v3 = v3 + &String::from(v2);
                                            match init_itra.next() {
                                                Some(next_char_value) => v2 = next_char_value,
                                                None => break,
                                            }
                                        }
                                        if variant == "" {
                                            let v4 = v.to_string() + &d1.to_string() + &v3;
                                            effects.push((v4, 1));
                                        } else {
                                            let v4 = variant.to_string()
                                                + &v.to_string()
                                                + &d1.to_string()
                                                + &v3;
                                            effects.push((v4, 1));
                                        }
                                    }
                                    _ => (),
                                }
                            }
                            'p' => {
                                let d1 = init_itra.next().unwrap();
                                match d1 {
                                    '-' => {
                                        //'p-'
                                        let mut v2 = init_itra.next().unwrap();
                                        let mut v3 = String::new();
                                        loop {
                                            v3 = v3 + &String::from(v2);
                                            match init_itra.next() {
                                                Some(next_char_value) => v2 = next_char_value,
                                                None => break,
                                            }
                                        }
                                        if variant == "" {
                                            let v4 = v.to_string() + &d1.to_string() + &v3;
                                            box_model.push((v4, 11));
                                        } else {
                                            let v4 = variant.to_string()
                                                + &v.to_string()
                                                + &d1.to_string()
                                                + &v3;
                                            box_model.push((v4, 11));
                                        }
                                    }
                                    'x' => {
                                        //'px-'
                                        let mut v2 = init_itra.next().unwrap();
                                        let mut v3 = String::new();
                                        loop {
                                            v3 = v3 + &String::from(v2);
                                            match init_itra.next() {
                                                Some(next_char_value) => v2 = next_char_value,
                                                None => break,
                                            }
                                        }
                                        if variant == "" {
                                            let v4 = v.to_string() + &d1.to_string() + &v3;
                                            box_model.push((v4, 12));
                                        } else {
                                            let v4 = variant.to_string()
                                                + &v.to_string()
                                                + &d1.to_string()
                                                + &v3;
                                            box_model.push((v4, 12));
                                        }
                                    }
                                    'y' => {
                                        //'py-'
                                        let mut v2 = init_itra.next().unwrap();
                                        let mut v3 = String::new();
                                        loop {
                                            v3 = v3 + &String::from(v2);
                                            match init_itra.next() {
                                                Some(next_char_value) => v2 = next_char_value,
                                                None => break,
                                            }
                                        }
                                        if variant == "" {
                                            let v4 = v.to_string() + &d1.to_string() + &v3;
                                            box_model.push((v4, 13));
                                        } else {
                                            let v4 = variant.to_string()
                                                + &v.to_string()
                                                + &d1.to_string()
                                                + &v3;
                                            box_model.push((v4, 13));
                                        }
                                    }
                                    't' => {
                                        //'pt-'
                                        let mut v2 = init_itra.next().unwrap();
                                        let mut v3 = String::new();
                                        loop {
                                            v3 = v3 + &String::from(v2);
                                            match init_itra.next() {
                                                Some(next_char_value) => v2 = next_char_value,
                                                None => break,
                                            }
                                        }
                                        if variant == "" {
                                            let v4 = v.to_string() + &d1.to_string() + &v3;
                                            box_model.push((v4, 14));
                                        } else {
                                            let v4 = variant.to_string()
                                                + &v.to_string()
                                                + &d1.to_string()
                                                + &v3;
                                            box_model.push((v4, 14));
                                        }
                                    }
                                    'l' => {
                                        let d2 = init_itra.next().unwrap();
                                        match d2 {
                                            '-' => {
                                                //'pl-'
                                                let mut v2 = init_itra.next().unwrap();
                                                let mut v3 = String::new();
                                                loop {
                                                    v3 = v3 + &String::from(v2);
                                                    match init_itra.next() {
                                                        Some(next_char_value) => {
                                                            v2 = next_char_value
                                                        }
                                                        None => break,
                                                    }
                                                }
                                                if variant == "" {
                                                    let v4 = v.to_string()
                                                        + &d1.to_string()
                                                        + &d2.to_string()
                                                        + &v3;
                                                    box_model.push((v4, 15));
                                                } else {
                                                    let v4 = variant.to_string()
                                                        + &v.to_string()
                                                        + &d1.to_string()
                                                        + &d2.to_string()
                                                        + &v3;
                                                    box_model.push((v4, 15));
                                                }
                                            }
                                            'a' => {
                                                let d3 = init_itra.next().unwrap();
                                                match d3 {
                                                    'c' => {
                                                        let d4 = init_itra.next().unwrap();
                                                        match d4 {
                                                            'e' => {
                                                                let d5 = init_itra.next().unwrap();
                                                                match d5 {
                                                                    '-' => {
                                                                        let d6 = init_itra
                                                                            .next()
                                                                            .unwrap();
                                                                        match d6 {
                                                                            'c' => {
                                                                                //'place-content'
                                                                                let mut v2 =
                                                                                    init_itra
                                                                                        .next()
                                                                                        .unwrap();
                                                                                let mut v3 =
                                                                                    String::new();
                                                                                loop {
                                                                                    v3 = v3 + &String::from(v2);
                                                                                    match init_itra.next() {
    																	        Some(next_char_value) => v2 = next_char_value,
    																	        None => break,
    																	    }
                                                                                }
                                                                                if variant == "" {
                                                                                    let v4 = v.to_string() + &d1.to_string()+ &d2.to_string()+ &d3.to_string()+ &d4.to_string()+ &d5.to_string()+ &d6.to_string() + &v3;
                                                                                    alignment.push(
                                                                                        (v4, 3),
                                                                                    );
                                                                                } else {
                                                                                    let v4 = variant.to_string() + &v.to_string() + &d1.to_string()+ &d2.to_string()+ &d3.to_string()+ &d4.to_string()+ &d5.to_string()+ &d6.to_string() + &v3;
                                                                                    alignment.push(
                                                                                        (v4, 3),
                                                                                    );
                                                                                }
                                                                            }
                                                                            'i' => {
                                                                                //'place-items'
                                                                                let mut v2 =
                                                                                    init_itra
                                                                                        .next()
                                                                                        .unwrap();
                                                                                let mut v3 =
                                                                                    String::new();
                                                                                loop {
                                                                                    v3 = v3 + &String::from(v2);
                                                                                    match init_itra.next() {
    																	        Some(next_char_value) => v2 = next_char_value,
    																	        None => break,
    																	    }
                                                                                }
                                                                                if variant == "" {
                                                                                    let v4 = v.to_string() + &d1.to_string()+ &d2.to_string()+ &d3.to_string()+ &d4.to_string()+ &d5.to_string()+ &d6.to_string() + &v3;
                                                                                    alignment.push(
                                                                                        (v4, 5),
                                                                                    );
                                                                                } else {
                                                                                    let v4 = variant.to_string() + &v.to_string() + &d1.to_string()+ &d2.to_string()+ &d3.to_string()+ &d4.to_string()+ &d5.to_string()+ &d6.to_string() + &v3;
                                                                                    alignment.push(
                                                                                        (v4, 5),
                                                                                    );
                                                                                }
                                                                            }
                                                                            's' => {
                                                                                //'place-self'
                                                                                let mut v2 =
                                                                                    init_itra
                                                                                        .next()
                                                                                        .unwrap();
                                                                                let mut v3 =
                                                                                    String::new();
                                                                                loop {
                                                                                    v3 = v3 + &String::from(v2);
                                                                                    match init_itra.next() {
    																	        Some(next_char_value) => v2 = next_char_value,
    																	        None => break,
    																	    }
                                                                                }
                                                                                if variant == "" {
                                                                                    let v4 = v.to_string() + &d1.to_string()+ &d2.to_string()+ &d3.to_string()+ &d4.to_string()+ &d5.to_string()+ &d6.to_string() + &v3;
                                                                                    alignment.push(
                                                                                        (v4, 6),
                                                                                    );
                                                                                } else {
                                                                                    let v4 = variant.to_string() + &v.to_string() + &d1.to_string()+ &d2.to_string()+ &d3.to_string()+ &d4.to_string()+ &d5.to_string()+ &d6.to_string() + &v3;
                                                                                    alignment.push(
                                                                                        (v4, 6),
                                                                                    );
                                                                                }
                                                                            }
                                                                            _ => (),
                                                                        }
                                                                    }
                                                                    _ => (),
                                                                }
                                                            }
                                                            _ => (),
                                                        }
                                                    }
                                                    _ => (),
                                                }
                                            }
                                            _ => (),
                                        }
                                    }
                                    'r' => {
                                        let d2 = init_itra.next().unwrap();
                                        match d2 {
                                            '-' => {
                                                //'pr-'
                                                let mut v2 = init_itra.next().unwrap();
                                                let mut v3 = String::new();
                                                loop {
                                                    v3 = v3 + &String::from(v2);
                                                    match init_itra.next() {
                                                        Some(next_char_value) => {
                                                            v2 = next_char_value
                                                        }
                                                        None => break,
                                                    }
                                                }
                                                if variant == "" {
                                                    let v4 = v.to_string()
                                                        + &d1.to_string()
                                                        + &d2.to_string()
                                                        + &v3;
                                                    box_model.push((v4, 16));
                                                } else {
                                                    let v4 = variant.to_string()
                                                        + &v.to_string()
                                                        + &d1.to_string()
                                                        + &d2.to_string()
                                                        + &v3;
                                                    box_model.push((v4, 16));
                                                }
                                            }
                                            'o' => {
                                                //'proportional-nums'
                                                let mut v2 = init_itra.next().unwrap();
                                                let mut v3 = String::new();
                                                loop {
                                                    v3 = v3 + &String::from(v2);
                                                    match init_itra.next() {
                                                        Some(next_char_value) => {
                                                            v2 = next_char_value
                                                        }
                                                        None => break,
                                                    }
                                                }
                                                if variant == "" {
                                                    let v4 = v.to_string()
                                                        + &d1.to_string()
                                                        + &d2.to_string()
                                                        + &v3;
                                                    typography.push((v4, 9));
                                                } else {
                                                    let v4 = variant.to_string()
                                                        + &v.to_string()
                                                        + &d1.to_string()
                                                        + &d2.to_string()
                                                        + &v3;
                                                    typography.push((v4, 9));
                                                }
                                            }
                                            _ => (),
                                        }
                                    }
                                    'b' => {
                                        //'pb-'
                                        let mut v2 = init_itra.next().unwrap();
                                        let mut v3 = String::new();
                                        loop {
                                            v3 = v3 + &String::from(v2);
                                            match init_itra.next() {
                                                Some(next_char_value) => v2 = next_char_value,
                                                None => break,
                                            }
                                        }
                                        if variant == "" {
                                            let v4 = v.to_string() + &d1.to_string() + &v3;
                                            box_model.push((v4, 17));
                                        } else {
                                            let v4 = variant.to_string()
                                                + &v.to_string()
                                                + &d1.to_string()
                                                + &v3;
                                            box_model.push((v4, 17));
                                        }
                                    }
                                    'o' => {
                                        //'pointer'
                                        let mut v2 = init_itra.next().unwrap();
                                        let mut v3 = String::new();
                                        loop {
                                            v3 = v3 + &String::from(v2);
                                            match init_itra.next() {
                                                Some(next_char_value) => v2 = next_char_value,
                                                None => break,
                                            }
                                        }
                                        if variant == "" {
                                            let v4 = v.to_string() + &d1.to_string() + &v3;
                                            misc.push((v4, 1));
                                        } else {
                                            let v4 = variant.to_string()
                                                + &v.to_string()
                                                + &d1.to_string()
                                                + &v3;
                                            misc.push((v4, 1));
                                        }
                                    }
                                    _ => (),
                                }
                            }
                            'r' => {
                                let d1 = init_itra.next().unwrap();
                                match d1 {
                                    'e' => {
                                        let d2 = init_itra.next().unwrap();
                                        match d2 {
                                            'l' => {
                                                //'relative'
                                                let mut v2 = init_itra.next().unwrap();
                                                let mut v3 = String::new();
                                                loop {
                                                    v3 = v3 + &String::from(v2);
                                                    match init_itra.next() {
                                                        Some(next_char_value) => {
                                                            v2 = next_char_value
                                                        }
                                                        None => break,
                                                    }
                                                }
                                                if variant == "" {
                                                    let v4 = v.to_string()
                                                        + &d1.to_string()
                                                        + &d2.to_string()
                                                        + &v3;
                                                    positioning.push((v4, 1));
                                                } else {
                                                    let v4 = variant.to_string()
                                                        + &v.to_string()
                                                        + &d1.to_string()
                                                        + &d2.to_string()
                                                        + &v3;
                                                    positioning.push((v4, 1));
                                                }
                                            }
                                            's' => {
                                                //'resize'
                                                let mut v2 = init_itra.next().unwrap();
                                                let mut v3 = String::new();
                                                loop {
                                                    v3 = v3 + &String::from(v2);
                                                    match init_itra.next() {
                                                        Some(next_char_value) => {
                                                            v2 = next_char_value
                                                        }
                                                        None => break,
                                                    }
                                                }
                                                if variant == "" {
                                                    let v4 = v.to_string()
                                                        + &d1.to_string()
                                                        + &d2.to_string()
                                                        + &v3;
                                                    misc.push((v4, 1));
                                                } else {
                                                    let v4 = variant.to_string()
                                                        + &v.to_string()
                                                        + &d1.to_string()
                                                        + &d2.to_string()
                                                        + &v3;
                                                    misc.push((v4, 1));
                                                }
                                            }
                                            _ => (),
                                        }
                                    }
                                    'i' => {
                                        let d2 = init_itra.next().unwrap();
                                        match d2 {
                                            'g' => {
                                                //'right'
                                                let mut v2 = init_itra.next().unwrap();
                                                let mut v3 = String::new();
                                                loop {
                                                    v3 = v3 + &String::from(v2);
                                                    match init_itra.next() {
                                                        Some(next_char_value) => {
                                                            v2 = next_char_value
                                                        }
                                                        None => break,
                                                    }
                                                }
                                                if variant == "" {
                                                    let v4 = v.to_string()
                                                        + &d1.to_string()
                                                        + &d2.to_string()
                                                        + &v3;
                                                    positioning.push((v4, 4));
                                                } else {
                                                    let v4 = variant.to_string()
                                                        + &v.to_string()
                                                        + &d1.to_string()
                                                        + &d2.to_string()
                                                        + &v3;
                                                    positioning.push((v4, 4));
                                                }
                                            }
                                            'n' => {
                                                //'ring'
                                                let mut v2 = init_itra.next().unwrap();
                                                let mut v3 = String::new();
                                                loop {
                                                    v3 = v3 + &String::from(v2);
                                                    match init_itra.next() {
                                                        Some(next_char_value) => {
                                                            v2 = next_char_value
                                                        }
                                                        None => break,
                                                    }
                                                }
                                                if variant == "" {
                                                    let v4 = v.to_string()
                                                        + &d1.to_string()
                                                        + &d2.to_string()
                                                        + &v3;
                                                    box_model.push((v4, 23));
                                                } else {
                                                    let v4 = variant.to_string()
                                                        + &v.to_string()
                                                        + &d1.to_string()
                                                        + &d2.to_string()
                                                        + &v3;
                                                    box_model.push((v4, 23));
                                                }
                                            }
                                            _ => (),
                                        }
                                    }
                                    'o' => {
                                        let d2 = init_itra.next().unwrap();
                                        match d2 {
                                            'u' => {
                                                //'rounded'
                                                let mut v2 = init_itra.next().unwrap();
                                                let mut v3 = String::new();
                                                loop {
                                                    v3 = v3 + &String::from(v2);
                                                    match init_itra.next() {
                                                        Some(next_char_value) => {
                                                            v2 = next_char_value
                                                        }
                                                        None => break,
                                                    }
                                                }
                                                if variant == "" {
                                                    let v4 = v.to_string()
                                                        + &d1.to_string()
                                                        + &d2.to_string()
                                                        + &v3;
                                                    box_model.push((v4, 32));
                                                } else {
                                                    let v4 = variant.to_string()
                                                        + &v.to_string()
                                                        + &d1.to_string()
                                                        + &d2.to_string()
                                                        + &v3;
                                                    box_model.push((v4, 32));
                                                }
                                            }
                                            'w' => {
                                                //'row'
                                                let mut v2 = init_itra.next().unwrap();
                                                let mut v3 = String::new();
                                                loop {
                                                    v3 = v3 + &String::from(v2);
                                                    match init_itra.next() {
                                                        Some(next_char_value) => {
                                                            v2 = next_char_value
                                                        }
                                                        None => break,
                                                    }
                                                }
                                                if variant == "" {
                                                    let v4 = v.to_string()
                                                        + &d1.to_string()
                                                        + &d2.to_string()
                                                        + &v3;
                                                    flex_grid.push((v4, 2));
                                                } else {
                                                    let v4 = variant.to_string()
                                                        + &v.to_string()
                                                        + &d1.to_string()
                                                        + &d2.to_string()
                                                        + &v3;
                                                    flex_grid.push((v4, 2));
                                                }
                                            }
                                            't' => {
                                                //'rotate'
                                                let mut v2 = init_itra.next().unwrap();
                                                let mut v3 = String::new();
                                                loop {
                                                    v3 = v3 + &String::from(v2);
                                                    match init_itra.next() {
                                                        Some(next_char_value) => {
                                                            v2 = next_char_value
                                                        }
                                                        None => break,
                                                    }
                                                }
                                                if variant == "" {
                                                    let v4 = v.to_string()
                                                        + &d1.to_string()
                                                        + &d2.to_string()
                                                        + &v3;
                                                    transforms.push((v4, 5));
                                                } else {
                                                    let v4 = variant.to_string()
                                                        + &v.to_string()
                                                        + &d1.to_string()
                                                        + &d2.to_string()
                                                        + &v3;
                                                    transforms.push((v4, 5));
                                                }
                                            }
                                            _ => (),
                                        }
                                    }
                                    _ => (),
                                }
                            }
                            's' => {
                                let d1 = init_itra.next().unwrap();
                                match d1 {
                                    't' => {
                                        let d2 = init_itra.next().unwrap();
                                        match d2 {
                                            'a' => {
                                                let d3 = init_itra.next().unwrap();
                                                match d3 {
                                                    't' => {
                                                        //'static'
                                                        let mut v2 = init_itra.next().unwrap();
                                                        let mut v3 = String::new();
                                                        loop {
                                                            v3 = v3 + &String::from(v2);
                                                            match init_itra.next() {
                                                                Some(next_char_value) => {
                                                                    v2 = next_char_value
                                                                }
                                                                None => break,
                                                            }
                                                        }
                                                        if variant == "" {
                                                            let v4 = v.to_string()
                                                                + &d1.to_string()
                                                                + &d2.to_string()
                                                                + &d3.to_string()
                                                                + &v3;
                                                            positioning.push((v4, 1));
                                                        } else {
                                                            let v4 = variant.to_string()
                                                                + &v.to_string()
                                                                + &d1.to_string()
                                                                + &d2.to_string()
                                                                + &d3.to_string()
                                                                + &v3;
                                                            positioning.push((v4, 1));
                                                        }
                                                    }
                                                    'c' => {
                                                        //'stacked-fractions'
                                                        let mut v2 = init_itra.next().unwrap();
                                                        let mut v3 = String::new();
                                                        loop {
                                                            v3 = v3 + &String::from(v2);
                                                            match init_itra.next() {
                                                                Some(next_char_value) => {
                                                                    v2 = next_char_value
                                                                }
                                                                None => break,
                                                            }
                                                        }
                                                        if variant == "" {
                                                            let v4 = v.to_string()
                                                                + &d1.to_string()
                                                                + &d2.to_string()
                                                                + &d3.to_string()
                                                                + &v3;
                                                            typography.push((v4, 9));
                                                        } else {
                                                            let v4 = variant.to_string()
                                                                + &v.to_string()
                                                                + &d1.to_string()
                                                                + &d2.to_string()
                                                                + &d3.to_string()
                                                                + &v3;
                                                            typography.push((v4, 9));
                                                        }
                                                    }
                                                    _ => (),
                                                }
                                            }
                                            'i' => {
                                                //'sticky'
                                                let mut v2 = init_itra.next().unwrap();
                                                let mut v3 = String::new();
                                                loop {
                                                    v3 = v3 + &String::from(v2);
                                                    match init_itra.next() {
                                                        Some(next_char_value) => {
                                                            v2 = next_char_value
                                                        }
                                                        None => break,
                                                    }
                                                }
                                                if variant == "" {
                                                    let v4 = v.to_string()
                                                        + &d1.to_string()
                                                        + &d2.to_string()
                                                        + &v3;
                                                    positioning.push((v4, 1));
                                                } else {
                                                    let v4 = variant.to_string()
                                                        + &v.to_string()
                                                        + &d1.to_string()
                                                        + &d2.to_string()
                                                        + &v3;
                                                    positioning.push((v4, 1));
                                                }
                                            }
                                            'r' => {
                                                let d3 = init_itra.next().unwrap();
                                                match d3 {
                                                    'i' => {
                                                        //'strike-through'
                                                        let mut v2 = init_itra.next().unwrap();
                                                        let mut v3 = String::new();
                                                        loop {
                                                            v3 = v3 + &String::from(v2);
                                                            match init_itra.next() {
                                                                Some(next_char_value) => {
                                                                    v2 = next_char_value
                                                                }
                                                                None => break,
                                                            }
                                                        }
                                                        if variant == "" {
                                                            let v4 = v.to_string()
                                                                + &d1.to_string()
                                                                + &d2.to_string()
                                                                + &d3.to_string()
                                                                + &v3;
                                                            typography.push((v4, 10));
                                                        } else {
                                                            let v4 = variant.to_string()
                                                                + &v.to_string()
                                                                + &d1.to_string()
                                                                + &d2.to_string()
                                                                + &d3.to_string()
                                                                + &v3;
                                                            typography.push((v4, 10));
                                                        }
                                                    }
                                                    'o' => {
                                                        //'stroke'
                                                        let mut v2 = init_itra.next().unwrap();
                                                        let mut v3 = String::new();
                                                        loop {
                                                            v3 = v3 + &String::from(v2);
                                                            match init_itra.next() {
                                                                Some(next_char_value) => {
                                                                    v2 = next_char_value
                                                                }
                                                                None => break,
                                                            }
                                                        }
                                                        if variant == "" {
                                                            let v4 = v.to_string()
                                                                + &d1.to_string()
                                                                + &d2.to_string()
                                                                + &d3.to_string()
                                                                + &v3;
                                                            visuals.push((v4, 2));
                                                        } else {
                                                            let v4 = variant.to_string()
                                                                + &v.to_string()
                                                                + &d1.to_string()
                                                                + &d2.to_string()
                                                                + &d3.to_string()
                                                                + &v3;
                                                            visuals.push((v4, 2));
                                                        }
                                                    }
                                                    _ => (),
                                                }
                                            }
                                            _ => (),
                                        }
                                    }
                                    'p' => {
                                        let d2 = init_itra.next().unwrap();
                                        match d2 {
                                            'a' => {
                                                let d3 = init_itra.next().unwrap();
                                                match d3 {
                                                    'c' => {
                                                        let d4 = init_itra.next().unwrap();
                                                        match d4 {
                                                            'e' => {
                                                                let d5 = init_itra.next().unwrap();
                                                                match d5 {
                                                                    '-' => {
                                                                        let d6 = init_itra
                                                                            .next()
                                                                            .unwrap();
                                                                        match d6 {
                                                                            'x' => {
                                                                                //'space-x'
                                                                                let mut v2 =
                                                                                    init_itra
                                                                                        .next()
                                                                                        .unwrap();
                                                                                let mut v3 =
                                                                                    String::new();
                                                                                loop {
                                                                                    v3 = v3 + &String::from(v2);
                                                                                    match init_itra.next() {
    																	        Some(next_char_value) => v2 = next_char_value,
    																	        None => break,
    																	    }
                                                                                }
                                                                                if variant == "" {
                                                                                    let v4 = v.to_string() + &d1.to_string()+ &d2.to_string()+ &d3.to_string()+ &d4.to_string()+ &d5.to_string()+ &d6.to_string() + &v3;
                                                                                    box_model.push(
                                                                                        (v4, 48),
                                                                                    );
                                                                                } else {
                                                                                    let v4 = variant.to_string() + &v.to_string() + &d1.to_string()+ &d2.to_string()+ &d3.to_string()+ &d4.to_string()+ &d5.to_string()+ &d6.to_string() + &v3;
                                                                                    box_model.push(
                                                                                        (v4, 48),
                                                                                    );
                                                                                }
                                                                            }
                                                                            'y' => {
                                                                                //'space-y'
                                                                                let mut v2 =
                                                                                    init_itra
                                                                                        .next()
                                                                                        .unwrap();
                                                                                let mut v3 =
                                                                                    String::new();
                                                                                loop {
                                                                                    v3 = v3 + &String::from(v2);
                                                                                    match init_itra.next() {
    																	        Some(next_char_value) => v2 = next_char_value,
    																	        None => break,
    																	    }
                                                                                }
                                                                                if variant == "" {
                                                                                    let v4 = v.to_string() + &d1.to_string()+ &d2.to_string()+ &d3.to_string()+ &d4.to_string()+ &d5.to_string()+ &d6.to_string() + &v3;
                                                                                    box_model.push(
                                                                                        (v4, 49),
                                                                                    );
                                                                                } else {
                                                                                    let v4 = variant.to_string() + &v.to_string() + &d1.to_string()+ &d2.to_string()+ &d3.to_string()+ &d4.to_string()+ &d5.to_string()+ &d6.to_string() + &v3;
                                                                                    box_model.push(
                                                                                        (v4, 49),
                                                                                    );
                                                                                }
                                                                            }
                                                                            _ => (),
                                                                        }
                                                                    }
                                                                    _ => (),
                                                                }
                                                            }
                                                            _ => (),
                                                        }
                                                    }
                                                    _ => (),
                                                }
                                            }
                                            _ => (),
                                        }
                                    }
                                    'e' => {
                                        let d2 = init_itra.next().unwrap();
                                        match d2 {
                                            'l' => {
                                                let d3 = init_itra.next().unwrap();
                                                match d3 {
                                                    'f' => {
                                                        //'self'
                                                        let mut v2 = init_itra.next().unwrap();
                                                        let mut v3 = String::new();
                                                        loop {
                                                            v3 = v3 + &String::from(v2);
                                                            match init_itra.next() {
                                                                Some(next_char_value) => {
                                                                    v2 = next_char_value
                                                                }
                                                                None => break,
                                                            }
                                                        }
                                                        if variant == "" {
                                                            let v4 = v.to_string()
                                                                + &d1.to_string()
                                                                + &d2.to_string()
                                                                + &d3.to_string()
                                                                + &v3;
                                                            alignment.push((v4, 4));
                                                        } else {
                                                            let v4 = variant.to_string()
                                                                + &v.to_string()
                                                                + &d1.to_string()
                                                                + &d2.to_string()
                                                                + &d3.to_string()
                                                                + &v3;
                                                            alignment.push((v4, 4));
                                                        }
                                                    }
                                                    'e' => {
                                                        //'select'
                                                        let mut v2 = init_itra.next().unwrap();
                                                        let mut v3 = String::new();
                                                        loop {
                                                            v3 = v3 + &String::from(v2);
                                                            match init_itra.next() {
                                                                Some(next_char_value) => {
                                                                    v2 = next_char_value
                                                                }
                                                                None => break,
                                                            }
                                                        }
                                                        if variant == "" {
                                                            let v4 = v.to_string()
                                                                + &d1.to_string()
                                                                + &d2.to_string()
                                                                + &d3.to_string()
                                                                + &v3;
                                                            misc.push((v4, 1));
                                                        } else {
                                                            let v4 = variant.to_string()
                                                                + &v.to_string()
                                                                + &d1.to_string()
                                                                + &d2.to_string()
                                                                + &d3.to_string()
                                                                + &v3;
                                                            misc.push((v4, 1));
                                                        }
                                                    }
                                                    _ => (),
                                                }
                                            }
                                            'p' => {
                                                //'sepia'
                                                let mut v2 = init_itra.next().unwrap();
                                                let mut v3 = String::new();
                                                loop {
                                                    v3 = v3 + &String::from(v2);
                                                    match init_itra.next() {
                                                        Some(next_char_value) => {
                                                            v2 = next_char_value
                                                        }
                                                        None => break,
                                                    }
                                                }
                                                if variant == "" {
                                                    let v4 = v.to_string()
                                                        + &d1.to_string()
                                                        + &d2.to_string()
                                                        + &v3;
                                                    effects.push((v4, 10));
                                                } else {
                                                    let v4 = variant.to_string()
                                                        + &v.to_string()
                                                        + &d1.to_string()
                                                        + &d2.to_string()
                                                        + &v3;
                                                    effects.push((v4, 10));
                                                }
                                            }
                                            _ => (),
                                        }
                                    }
                                    'h' => {
                                        let d2 = init_itra.next().unwrap();
                                        match d2 {
                                            'r' => {
                                                //'shrink'
                                                let mut v2 = init_itra.next().unwrap();
                                                let mut v3 = String::new();
                                                loop {
                                                    v3 = v3 + &String::from(v2);
                                                    match init_itra.next() {
                                                        Some(next_char_value) => {
                                                            v2 = next_char_value
                                                        }
                                                        None => break,
                                                    }
                                                }
                                                if variant == "" {
                                                    let v4 = v.to_string()
                                                        + &d1.to_string()
                                                        + &d2.to_string()
                                                        + &v3;
                                                    flex_grid.push((v4, 3));
                                                } else {
                                                    let v4 = variant.to_string()
                                                        + &v.to_string()
                                                        + &d1.to_string()
                                                        + &d2.to_string()
                                                        + &v3;
                                                    flex_grid.push((v4, 3));
                                                }
                                            }
                                            'a' => {
                                                //'shadow'
                                                let mut v2 = init_itra.next().unwrap();
                                                let mut v3 = String::new();
                                                loop {
                                                    v3 = v3 + &String::from(v2);
                                                    match init_itra.next() {
                                                        Some(next_char_value) => {
                                                            v2 = next_char_value
                                                        }
                                                        None => break,
                                                    }
                                                }
                                                if variant == "" {
                                                    let v4 = v.to_string()
                                                        + &d1.to_string()
                                                        + &d2.to_string()
                                                        + &v3;
                                                    effects.push((v4, 1));
                                                } else {
                                                    let v4 = variant.to_string()
                                                        + &v.to_string()
                                                        + &d1.to_string()
                                                        + &d2.to_string()
                                                        + &v3;
                                                    effects.push((v4, 1));
                                                }
                                            }
                                            _ => (),
                                        }
                                    }
                                    'u' => {
                                        //'subpixel-antialiased'
                                        let mut v2 = init_itra.next().unwrap();
                                        let mut v3 = String::new();
                                        loop {
                                            v3 = v3 + &String::from(v2);
                                            match init_itra.next() {
                                                Some(next_char_value) => v2 = next_char_value,
                                                None => break,
                                            }
                                        }
                                        if variant == "" {
                                            let v4 = v.to_string() + &d1.to_string() + &v3;
                                            typography.push((v4, 4));
                                        } else {
                                            let v4 = variant.to_string()
                                                + &v.to_string()
                                                + &d1.to_string()
                                                + &v3;
                                            typography.push((v4, 4));
                                        }
                                    }
                                    'l' => {
                                        //'slashed-zero'
                                        let mut v2 = init_itra.next().unwrap();
                                        let mut v3 = String::new();
                                        loop {
                                            v3 = v3 + &String::from(v2);
                                            match init_itra.next() {
                                                Some(next_char_value) => v2 = next_char_value,
                                                None => break,
                                            }
                                        }
                                        if variant == "" {
                                            let v4 = v.to_string() + &d1.to_string() + &v3;
                                            typography.push((v4, 9));
                                        } else {
                                            let v4 = variant.to_string()
                                                + &v.to_string()
                                                + &d1.to_string()
                                                + &v3;
                                            typography.push((v4, 9));
                                        }
                                    }
                                    'a' => {
                                        //'saturate'
                                        let mut v2 = init_itra.next().unwrap();
                                        let mut v3 = String::new();
                                        loop {
                                            v3 = v3 + &String::from(v2);
                                            match init_itra.next() {
                                                Some(next_char_value) => v2 = next_char_value,
                                                None => break,
                                            }
                                        }
                                        if variant == "" {
                                            let v4 = v.to_string() + &d1.to_string() + &v3;
                                            effects.push((v4, 9));
                                        } else {
                                            let v4 = variant.to_string()
                                                + &v.to_string()
                                                + &d1.to_string()
                                                + &v3;
                                            effects.push((v4, 9));
                                        }
                                    }
                                    'c' => {
                                        let d2 = init_itra.next().unwrap();
                                        match d2 {
                                            'a' => {
                                                //'scale'
                                                let mut v2 = init_itra.next().unwrap();
                                                let mut v3 = String::new();
                                                loop {
                                                    v3 = v3 + &String::from(v2);
                                                    match init_itra.next() {
                                                        Some(next_char_value) => {
                                                            v2 = next_char_value
                                                        }
                                                        None => break,
                                                    }
                                                }
                                                if variant == "" {
                                                    let v4 = v.to_string()
                                                        + &d1.to_string()
                                                        + &d2.to_string()
                                                        + &v3;
                                                    transforms.push((v4, 3));
                                                } else {
                                                    let v4 = variant.to_string()
                                                        + &v.to_string()
                                                        + &d1.to_string()
                                                        + &d2.to_string()
                                                        + &v3;
                                                    transforms.push((v4, 3));
                                                }
                                            }
                                            'o' => {
                                                //'scoll'
                                                let mut v2 = init_itra.next().unwrap();
                                                let mut v3 = String::new();
                                                loop {
                                                    v3 = v3 + &String::from(v2);
                                                    match init_itra.next() {
                                                        Some(next_char_value) => {
                                                            v2 = next_char_value
                                                        }
                                                        None => break,
                                                    }
                                                }
                                                if variant == "" {
                                                    let v4 = v.to_string()
                                                        + &d1.to_string()
                                                        + &d2.to_string()
                                                        + &v3;
                                                    misc.push((v4, 1));
                                                } else {
                                                    let v4 = variant.to_string()
                                                        + &v.to_string()
                                                        + &d1.to_string()
                                                        + &d2.to_string()
                                                        + &v3;
                                                    misc.push((v4, 1));
                                                }
                                            }
                                            _ => (),
                                        }
                                    }
                                    'k' => {
                                        //'skew'
                                        let mut v2 = init_itra.next().unwrap();
                                        let mut v3 = String::new();
                                        loop {
                                            v3 = v3 + &String::from(v2);
                                            match init_itra.next() {
                                                Some(next_char_value) => v2 = next_char_value,
                                                None => break,
                                            }
                                        }
                                        if variant == "" {
                                            let v4 = v.to_string() + &d1.to_string() + &v3;
                                            transforms.push((v4, 4));
                                        } else {
                                            let v4 = variant.to_string()
                                                + &v.to_string()
                                                + &d1.to_string()
                                                + &v3;
                                            transforms.push((v4, 4));
                                        }
                                    }
                                    'n' => {
                                        //'snap'
                                        let mut v2 = init_itra.next().unwrap();
                                        let mut v3 = String::new();
                                        loop {
                                            v3 = v3 + &String::from(v2);
                                            match init_itra.next() {
                                                Some(next_char_value) => v2 = next_char_value,
                                                None => break,
                                            }
                                        }
                                        if variant == "" {
                                            let v4 = v.to_string() + &d1.to_string() + &v3;
                                            misc.push((v4, 1));
                                        } else {
                                            let v4 = variant.to_string()
                                                + &v.to_string()
                                                + &d1.to_string()
                                                + &v3;
                                            misc.push((v4, 1));
                                        }
                                    }
                                    'r' => {
                                        //'sr-only'
                                        let mut v2 = init_itra.next().unwrap();
                                        let mut v3 = String::new();
                                        loop {
                                            v3 = v3 + &String::from(v2);
                                            match init_itra.next() {
                                                Some(next_char_value) => v2 = next_char_value,
                                                None => break,
                                            }
                                        }
                                        if variant == "" {
                                            let v4 = v.to_string() + &d1.to_string() + &v3;
                                            misc.push((v4, 1));
                                        } else {
                                            let v4 = variant.to_string()
                                                + &v.to_string()
                                                + &d1.to_string()
                                                + &v3;
                                            misc.push((v4, 1));
                                        }
                                    }
                                    _ => (),
                                }
                            }
                            't' => {
                                let d1 = init_itra.next().unwrap();
                                match d1 {
                                    'o' => {
                                        let d2 = init_itra.next().unwrap();
                                        match d2 {
                                            'p' => {
                                                //'top'
                                                let mut v2 = init_itra.next().unwrap();
                                                let mut v3 = String::new();
                                                loop {
                                                    v3 = v3 + &String::from(v2);
                                                    match init_itra.next() {
                                                        Some(next_char_value) => {
                                                            v2 = next_char_value
                                                        }
                                                        None => break,
                                                    }
                                                }
                                                if variant == "" {
                                                    let v4 = v.to_string()
                                                        + &d1.to_string()
                                                        + &d2.to_string()
                                                        + &v3;
                                                    positioning.push((v4, 3));
                                                } else {
                                                    let v4 = variant.to_string()
                                                        + &v.to_string()
                                                        + &d1.to_string()
                                                        + &d2.to_string()
                                                        + &v3;
                                                    positioning.push((v4, 3));
                                                }
                                            }
                                            '-' => {
                                                //'to-'
                                                let mut v2 = init_itra.next().unwrap();
                                                let mut v3 = String::new();
                                                loop {
                                                    v3 = v3 + &String::from(v2);
                                                    match init_itra.next() {
                                                        Some(next_char_value) => {
                                                            v2 = next_char_value
                                                        }
                                                        None => break,
                                                    }
                                                }
                                                if variant == "" {
                                                    let v4 = v.to_string()
                                                        + &d1.to_string()
                                                        + &d2.to_string()
                                                        + &v3;
                                                    visuals.push((v4, 14));
                                                } else {
                                                    let v4 = variant.to_string()
                                                        + &v.to_string()
                                                        + &d1.to_string()
                                                        + &d2.to_string()
                                                        + &v3;
                                                    visuals.push((v4, 14));
                                                }
                                            }
                                            'u' => {
                                                //'touch'
                                                let mut v2 = init_itra.next().unwrap();
                                                let mut v3 = String::new();
                                                loop {
                                                    v3 = v3 + &String::from(v2);
                                                    match init_itra.next() {
                                                        Some(next_char_value) => {
                                                            v2 = next_char_value
                                                        }
                                                        None => break,
                                                    }
                                                }
                                                if variant == "" {
                                                    let v4 = v.to_string()
                                                        + &d1.to_string()
                                                        + &d2.to_string()
                                                        + &v3;
                                                    misc.push((v4, 1));
                                                } else {
                                                    let v4 = variant.to_string()
                                                        + &v.to_string()
                                                        + &d1.to_string()
                                                        + &d2.to_string()
                                                        + &v3;
                                                    misc.push((v4, 1));
                                                }
                                            }
                                            _ => (),
                                        }
                                    }
                                    'a' => {
                                        let d2 = init_itra.next().unwrap();
                                        match d2 {
                                            'b' => {
                                                let d3 = init_itra.next().unwrap();
                                                match d3 {
                                                    'l' => {
                                                        //'table'
                                                        let mut v2 = init_itra.next().unwrap();
                                                        let mut v3 = String::new();
                                                        loop {
                                                            v3 = v3 + &String::from(v2);
                                                            match init_itra.next() {
                                                                Some(next_char_value) => {
                                                                    v2 = next_char_value
                                                                }
                                                                None => break,
                                                            }
                                                        }
                                                        if variant == "" {
                                                            let v4 = v.to_string()
                                                                + &d1.to_string()
                                                                + &d2.to_string()
                                                                + &d3.to_string()
                                                                + &v3;
                                                            box_model.push((v4, 1));
                                                        } else {
                                                            let v4 = variant.to_string()
                                                                + &v.to_string()
                                                                + &d1.to_string()
                                                                + &d2.to_string()
                                                                + &d3.to_string()
                                                                + &v3;
                                                            box_model.push((v4, 1));
                                                        }
                                                    }
                                                    'u' => {
                                                        //'tabular-nums'
                                                        let mut v2 = init_itra.next().unwrap();
                                                        let mut v3 = String::new();
                                                        loop {
                                                            v3 = v3 + &String::from(v2);
                                                            match init_itra.next() {
                                                                Some(next_char_value) => {
                                                                    v2 = next_char_value
                                                                }
                                                                None => break,
                                                            }
                                                        }
                                                        if variant == "" {
                                                            let v4 = v.to_string()
                                                                + &d1.to_string()
                                                                + &d2.to_string()
                                                                + &d3.to_string()
                                                                + &v3;
                                                            typography.push((v4, 9));
                                                        } else {
                                                            let v4 = variant.to_string()
                                                                + &v.to_string()
                                                                + &d1.to_string()
                                                                + &d2.to_string()
                                                                + &d3.to_string()
                                                                + &v3;
                                                            typography.push((v4, 9));
                                                        }
                                                    }
                                                    _ => (),
                                                }
                                            }
                                            _ => (),
                                        }
                                    }
                                    'e' => {
                                        //'text-'
                                        let mut v2 = init_itra.next().unwrap();
                                        let mut v3 = String::new();
                                        loop {
                                            v3 = v3 + &String::from(v2);
                                            match init_itra.next() {
                                                Some(next_char_value) => v2 = next_char_value,
                                                None => break,
                                            }
                                        }
                                        if variant == "" {
                                            let v4 = v.to_string() + &d1.to_string() + &v3;
                                            typography.push((v4, 2));
                                        } else {
                                            let v4 = variant.to_string()
                                                + &v.to_string()
                                                + &d1.to_string()
                                                + &v3;
                                            typography.push((v4, 2));
                                        }
                                    }
                                    'r' => {
                                        let d2 = init_itra.next().unwrap();
                                        match d2 {
                                            'a' => {
                                                let d3 = init_itra.next().unwrap();
                                                match d3 {
                                                    'c' => {
                                                        //'tracking'
                                                        let mut v2 = init_itra.next().unwrap();
                                                        let mut v3 = String::new();
                                                        loop {
                                                            v3 = v3 + &String::from(v2);
                                                            match init_itra.next() {
                                                                Some(next_char_value) => {
                                                                    v2 = next_char_value
                                                                }
                                                                None => break,
                                                            }
                                                        }
                                                        if variant == "" {
                                                            let v4 = v.to_string()
                                                                + &d1.to_string()
                                                                + &d2.to_string()
                                                                + &d3.to_string()
                                                                + &v3;
                                                            typography.push((v4, 7));
                                                        } else {
                                                            let v4 = variant.to_string()
                                                                + &v.to_string()
                                                                + &d1.to_string()
                                                                + &d2.to_string()
                                                                + &d3.to_string()
                                                                + &v3;
                                                            typography.push((v4, 7));
                                                        }
                                                    }
                                                    'n' => {
                                                        let d4 = init_itra.next().unwrap();
                                                        match d4 {
                                                            's' => {
                                                                let d5 = init_itra.next().unwrap();
                                                                match d5 {
                                                                    'l' => {
                                                                        //'translate'
                                                                        let mut v2 = init_itra
                                                                            .next()
                                                                            .unwrap();
                                                                        let mut v3 = String::new();
                                                                        loop {
                                                                            v3 = v3
                                                                                + &String::from(v2);
                                                                            match init_itra.next() {
                                                                                Some(
                                                                                    next_char_value,
                                                                                ) => v2 =
                                                                                    next_char_value,
                                                                                None => break,
                                                                            }
                                                                        }
                                                                        if variant == "" {
                                                                            let v4 = v.to_string()
                                                                                + &d1.to_string()
                                                                                + &d2.to_string()
                                                                                + &d3.to_string()
                                                                                + &d4.to_string()
                                                                                + &d5.to_string()
                                                                                + &v3;
                                                                            transforms
                                                                                .push((v4, 2));
                                                                        } else {
                                                                            let v4 = variant
                                                                                .to_string()
                                                                                + &v.to_string()
                                                                                + &d1.to_string()
                                                                                + &d2.to_string()
                                                                                + &d3.to_string()
                                                                                + &d4.to_string()
                                                                                + &d5.to_string()
                                                                                + &v3;
                                                                            transforms
                                                                                .push((v4, 2));
                                                                        }
                                                                    }
                                                                    'i' => {
                                                                        //'transition'
                                                                        let mut v2 = init_itra
                                                                            .next()
                                                                            .unwrap();
                                                                        let mut v3 = String::new();
                                                                        loop {
                                                                            v3 = v3
                                                                                + &String::from(v2);
                                                                            match init_itra.next() {
                                                                                Some(
                                                                                    next_char_value,
                                                                                ) => v2 =
                                                                                    next_char_value,
                                                                                None => break,
                                                                            }
                                                                        }
                                                                        if variant == "" {
                                                                            let v4 = v.to_string()
                                                                                + &d1.to_string()
                                                                                + &d2.to_string()
                                                                                + &d3.to_string()
                                                                                + &d4.to_string()
                                                                                + &d5.to_string()
                                                                                + &v3;
                                                                            motion.push((v4, 2));
                                                                        } else {
                                                                            let v4 = variant
                                                                                .to_string()
                                                                                + &v.to_string()
                                                                                + &d1.to_string()
                                                                                + &d2.to_string()
                                                                                + &d3.to_string()
                                                                                + &d4.to_string()
                                                                                + &d5.to_string()
                                                                                + &v3;
                                                                            motion.push((v4, 2));
                                                                        }
                                                                    }
                                                                    _ => (),
                                                                }
                                                            }
                                                            _ => (),
                                                        }
                                                    }
                                                    _ => (),
                                                }
                                            }
                                            'u' => {
                                                //'truncate'
                                                let mut v2 = init_itra.next().unwrap();
                                                let mut v3 = String::new();
                                                loop {
                                                    v3 = v3 + &String::from(v2);
                                                    match init_itra.next() {
                                                        Some(next_char_value) => {
                                                            v2 = next_char_value
                                                        }
                                                        None => break,
                                                    }
                                                }
                                                if variant == "" {
                                                    let v4 = v.to_string()
                                                        + &d1.to_string()
                                                        + &d2.to_string()
                                                        + &v3;
                                                    typography.push((v4, 12));
                                                } else {
                                                    let v4 = variant.to_string()
                                                        + &v.to_string()
                                                        + &d1.to_string()
                                                        + &d2.to_string()
                                                        + &v3;
                                                    typography.push((v4, 12));
                                                }
                                            }
                                            _ => (),
                                        }
                                    }
                                    _ => (),
                                }
                            }
                            'u' => {
                                let d1 = init_itra.next().unwrap();
                                match d1 {
                                    'n' => {
                                        //'underline'
                                        let mut v2 = init_itra.next().unwrap();
                                        let mut v3 = String::new();
                                        loop {
                                            v3 = v3 + &String::from(v2);
                                            match init_itra.next() {
                                                Some(next_char_value) => v2 = next_char_value,
                                                None => break,
                                            }
                                        }
                                        if variant == "" {
                                            let v4 = v.to_string() + &d1.to_string() + &v3;
                                            typography.push((v4, 10));
                                        } else {
                                            let v4 = variant.to_string()
                                                + &v.to_string()
                                                + &d1.to_string()
                                                + &v3;
                                            typography.push((v4, 10));
                                        }
                                    }
                                    'p' => {
                                        //'uppercase'
                                        let mut v2 = init_itra.next().unwrap();
                                        let mut v3 = String::new();
                                        loop {
                                            v3 = v3 + &String::from(v2);
                                            match init_itra.next() {
                                                Some(next_char_value) => v2 = next_char_value,
                                                None => break,
                                            }
                                        }
                                        if variant == "" {
                                            let v4 = v.to_string() + &d1.to_string() + &v3;
                                            typography.push((v4, 11));
                                        } else {
                                            let v4 = variant.to_string()
                                                + &v.to_string()
                                                + &d1.to_string()
                                                + &v3;
                                            typography.push((v4, 11));
                                        }
                                    }
                                    _ => (),
                                }
                            }
                            'v' => {
                                let d1 = init_itra.next().unwrap();
                                match d1 {
                                    _ => (),
                                }
                            }
                            'w' => {
                                let d1 = init_itra.next().unwrap();
                                match d1 {
                                    '-' => {
                                        //'w-'
                                        let mut v2 = init_itra.next().unwrap();
                                        let mut v3 = String::new();
                                        loop {
                                            v3 = v3 + &String::from(v2);
                                            match init_itra.next() {
                                                Some(next_char_value) => v2 = next_char_value,
                                                None => break,
                                            }
                                        }
                                        if variant == "" {
                                            let v4 = v.to_string() + &d1.to_string() + &v3;
                                            box_model.push((v4, 4));
                                        } else {
                                            let v4 = variant.to_string()
                                                + &v.to_string()
                                                + &d1.to_string()
                                                + &v3;
                                            box_model.push((v4, 4));
                                        }
                                    }
                                    'h' => {
                                        //'whitespace'
                                        let mut v2 = init_itra.next().unwrap();
                                        let mut v3 = String::new();
                                        loop {
                                            v3 = v3 + &String::from(v2);
                                            match init_itra.next() {
                                                Some(next_char_value) => v2 = next_char_value,
                                                None => break,
                                            }
                                        }
                                        if variant == "" {
                                            let v4 = v.to_string() + &d1.to_string() + &v3;
                                            typography.push((v4, 15));
                                        } else {
                                            let v4 = variant.to_string()
                                                + &v.to_string()
                                                + &d1.to_string()
                                                + &v3;
                                            typography.push((v4, 15));
                                        }
                                    }
                                    'i' => {
                                        //'will-change'
                                        let mut v2 = init_itra.next().unwrap();
                                        let mut v3 = String::new();
                                        loop {
                                            v3 = v3 + &String::from(v2);
                                            match init_itra.next() {
                                                Some(next_char_value) => v2 = next_char_value,
                                                None => break,
                                            }
                                        }
                                        if variant == "" {
                                            let v4 = v.to_string() + &d1.to_string() + &v3;
                                            misc.push((v4, 1));
                                        } else {
                                            let v4 = variant.to_string()
                                                + &v.to_string()
                                                + &d1.to_string()
                                                + &v3;
                                            misc.push((v4, 1));
                                        }
                                    }
                                    _ => (),
                                }
                            }
                            'z' => {
                                let d1 = init_itra.next().unwrap();
                                match d1 {
                                    _ => (),
                                }
                            }
                            // _ => println!("_{}_", v),
                            _ => (),
                        }
                    }
                    None => (),
                }
                // print!("{}", v);
            }
            None => {
                // println!("Class Loop breaked");
                break;
            }
        }
    }

    // let duration = start.elapsed();
    box_model.sort_by(|a, b| {
        if a.1 < b.1 {
            Ordering::Less
        } else if a.1 == b.1 {
            Ordering::Equal
        } else {
            Ordering::Greater
        }
    });
    // println!("box_model {:#?}", box_model);

    positioning.sort_by(|a, b| {
        if a.1 < b.1 {
            Ordering::Less
        } else if a.1 == b.1 {
            Ordering::Equal
        } else {
            Ordering::Greater
        }
    });
    // println!("positioning {:#?}", positioning);

    alignment.sort_by(|a, b| {
        if a.1 < b.1 {
            Ordering::Less
        } else if a.1 == b.1 {
            Ordering::Equal
        } else {
            Ordering::Greater
        }
    });
    // println!("alignment {:#?}", alignment);

    flex_grid.sort_by(|a, b| {
        if a.1 < b.1 {
            Ordering::Less
        } else if a.1 == b.1 {
            Ordering::Equal
        } else {
            Ordering::Greater
        }
    });
    // println!("flex_grid {:#?}", flex_grid);

    typography.sort_by(|a, b| {
        if a.1 < b.1 {
            Ordering::Less
        } else if a.1 == b.1 {
            Ordering::Equal
        } else {
            Ordering::Greater
        }
    });
    // println!("typography {:#?}", typography);
    // Visuals :- Classes like bg, shadows, backdrop-shadows
    visuals.sort_by(|a, b| {
        if a.1 < b.1 {
            Ordering::Less
        } else if a.1 == b.1 {
            Ordering::Equal
        } else {
            Ordering::Greater
        }
    });
    // println!("visuals {:#?}", visuals);
    // Effects brightness blur filters as well mix
    effects.sort_by(|a, b| {
        if a.1 < b.1 {
            Ordering::Less
        } else if a.1 == b.1 {
            Ordering::Equal
        } else {
            Ordering::Greater
        }
    });
    // println!("effects {:#?}", effects);
    // Transform
    transforms.sort_by(|a, b| {
        if a.1 < b.1 {
            Ordering::Less
        } else if a.1 == b.1 {
            Ordering::Equal
        } else {
            Ordering::Greater
        }
    });
    // println!("transforms {:#?}", transforms);
    // ANimation & Transition
    motion.sort_by(|a, b| {
        if a.1 < b.1 {
            Ordering::Less
        } else if a.1 == b.1 {
            Ordering::Equal
        } else {
            Ordering::Greater
        }
    });
    // println!("motion {:#?}", motion);
    // Misc will contain classes like caret color or scroll snap classes;
    misc.sort_by(|a, b| {
        if a.1 < b.1 {
            Ordering::Less
        } else if a.1 == b.1 {
            Ordering::Equal
        } else {
            Ordering::Greater
        }
    });
    // println!("misc {:#?}", misc);

    box_model.extend(flex_grid);
    box_model.extend(alignment);
    box_model.extend(positioning);
    box_model.extend(typography);
    box_model.extend(visuals);
    box_model.extend(effects);
    box_model.extend(transforms);
    box_model.extend(motion);
    box_model.extend(misc);

    let mut result = String::new();

    for (a, _) in box_model {
        result += &(String::from(" ") + &a);
    }

    // println!("Total Loop Duration {:#?}", duration);
    result
}
