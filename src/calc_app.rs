//use std::fmt::format;

/// We derive Deserialize/Serialize so we can persist app state on shutdown.
//type Error = Box<dyn std::error::Error>;
use anyhow::anyhow;

#[derive(serde::Deserialize, serde::Serialize)]
pub struct Variable {
    name: String,
    value: f64,
    speed: f64,
    max_value: f64,
    min_value: f64,
    play: bool,
    direction: f64,
}

#[derive(serde::Deserialize, serde::Serialize)]
pub struct ExampleApp {
    equation: String,
    rad_deg: String,
    var_list: Vec<Variable>,
    //ans:f64
}

impl Default for ExampleApp {
    fn default() -> Self {
        Self {
            equation: "".to_owned(),
            rad_deg: "in radians".to_owned(),
            var_list: vec![],
            //ans:0.0,
        }
    }
}

impl egui::app::App for ExampleApp {
    /// Called each time the UI needs repainting, which may be many times per second.
    /// Put your widgets into a `SidePanel`, `TopPanel`, `CentralPanel`, `Window` or `Area`.

    fn ui(
        &mut self,
        ctx: &std::sync::Arc<egui::Context>,
        integration_context: &mut egui::app::IntegrationContext,
    ) {
        let ExampleApp {
            equation,
            rad_deg,
            var_list,
            //ans,
        } = self;

        // Example used in `README.md`

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("My Calculator");

            // ui.horizontal(|ui| {
            //     ui.label("Your name: ");
            //     ui.text_edit_singleline(name);
            //     ui.checkbox(show_label, "show label");

            //     *show_label ^= ui.button("flip").clicked
            // });

            // if name != "conor" && name != "anton" {
            //     ui.horizontal(|ui| {
            //         ui.label("Your name is not: conor");
            //     });
            // }
            // if *show_label {
            //     ui.label("hi");
            // }

            // ui.add(egui::Slider::u32(age, 0..=120).text("age"));
            // if ui.button("Click each year").clicked {
            //     *age += 1;
            // }

            // ui.label(format!("Hello '{}', age {}", name, *age - 1));

            // ui.add(egui::Slider::u32(length, 0..=200).text("längd cm"));

            ui.label("calculator");
            ui.text_edit_singleline(equation);
            ui.label(trim_calc(equation.to_string()));
            ui.label(" ");

            ui.horizontal(|ui| {
                if ui.button(format!("{}", rad_deg)).clicked {
                    if rad_deg == "in radians" {
                        *rad_deg = String::from("now in degrees");
                    } else {
                        *rad_deg = String::from("now in radians");
                    }
                }
            });
            if ui.button("+ varible").clicked {
                let var_names = vec!['b', 'c', 'd', 'f', 'h', 'j', 'k', 'l', 'm', 'n', 'o', 'p'];
                let new_var = Variable {
                    name: var_names[var_list.len()].to_string(),
                    value: 0.0,
                    speed: 0.0,
                    min_value: 0.0,
                    max_value: 200.0,
                    play: true,
                    direction: 1.0,
                };
                var_list.push(new_var);

                // *var_list = add_var(*variables);
            }

            for v in var_list.iter_mut() {
                ui.horizontal(|ui| {
                    ui.add(egui::Slider::f64(&mut v.value, v.min_value..=v.max_value));
                    ui.text_edit_singleline(&mut v.name);
                    ui.label(&v.value.to_string());
                });
                ui.collapsing("settings", |ui| {
                    ui.horizontal(|ui| {
                        ui.label("speed");
                        ui.add(egui::DragValue::f64(&mut v.speed));
                        v.play ^= ui.button("▶").clicked;
                    });
                    ui.horizontal(|ui| {
                        ui.label("min");
                        ui.add(egui::DragValue::f64(&mut v.min_value));
                        ui.label("max");
                        ui.add(egui::DragValue::f64(&mut v.max_value));
                    })
                });
                if *&v.play
                    && v.value + v.speed * v.direction <= v.max_value
                    && v.value + v.speed * v.direction >= v.min_value
                {
                    v.value += v.speed * v.direction;
                }
                if v.value + v.speed * v.direction > v.max_value
                    || v.value + v.speed * v.direction < v.min_value
                {
                    if v.direction == 1.0 {
                        v.value = v.max_value
                    } else {
                        v.value = v.min_value
                    }
                    v.direction *= -1.0
                }
            }

            ui.horizontal(|ui| {
                if ui.button("1").clicked {
                    equation.push_str("1");
                }
                if ui.button("2").clicked {
                    equation.push_str("2");
                }
                if ui.button("3").clicked {
                    equation.push_str("3");
                }
                if ui.button("4").clicked {
                    equation.push_str("4");
                }
                if ui.button("5").clicked {
                    equation.push_str("5");
                }
            });
            ui.horizontal(|ui| {
                if ui.button("6").clicked {
                    equation.push_str("6");
                }
                if ui.button("7").clicked {
                    equation.push_str("7");
                }
                if ui.button("8").clicked {
                    equation.push_str("8");
                }
                if ui.button("9").clicked {
                    equation.push_str("9");
                }
                if ui.button("0").clicked {
                    equation.push_str("0");
                }
            });
            ui.horizontal(|ui| {
                if ui.button("+").clicked {
                    equation.push_str("+");
                }
                if ui.button("-").clicked {
                    equation.push_str("-");
                }
                if ui.button("/").clicked {
                    equation.push_str("/");
                }
                if ui.button("*").clicked {
                    equation.push_str("*");
                }
                if ui.button(".").clicked {
                    equation.push_str(".");
                }
                if ui.button("^").clicked {
                    equation.push_str("^");
                }
            });
            ui.horizontal(|ui| {
                if ui.button("(").clicked {
                    equation.push_str("(");
                }
                if ui.button(")").clicked {
                    equation.push_str(")");
                }
            });

            // *show ^= ui.button("enter").clicked;
            if equation.len() != 0 {
                match calculate(&equation, var_list) {
                    Ok(answer) => ui.label(answer.to_string()),
                    Err(err) => ui.label(format!("ERROR: {} ", err)),
                };
            }
            // if *show {
            //     ui.label(calculate(&equation).to_string());
            // }

            ui.advance_cursor(16.0);

            ui.separator();

            if ui.button("Quit").clicked {
                integration_context.output.quit = true;
            }
        });

        integration_context.output.window_size = Some(ctx.used_size()); // resize the window to be just the size we need it to be
    }

    fn on_exit(&mut self, storage: &mut dyn egui::app::Storage) {
        egui::app::set_value(storage, egui::app::APP_KEY, self);
    }
}

fn calculate(equation: &str, variable: &Vec<Variable>) -> Result<f64, anyhow::Error> {
    //things to add: support for pi, sqrt, sin, cos, tan, a result history,
    //log, ln, e, maybe even i, error handling, graphs, sliding constants
    let text = &equation;
    let mut bracket_level;
    let start = 0;
    let end = text.len();

    if text.len() != 0 && start <= end {
        bracket_level = 0;
        for a in (start..end).rev() {
            let i: usize = a;
            if &text[i..i + 1] == ")" {
                bracket_level += 1;
            }
            if &text[i..i + 1] == "(" {
                bracket_level -= 1;
            }
            //dbg!(&text[i..i+1], bracket_level);
            if (&text[i..i + 1] == "-" || &text[i..i + 1] == "+") && bracket_level == 0 {
                if &text[i..i + 1] == "-" {
                    //dbg!("test", &*text, bracket_level);
                    return Ok(calculate(&text[start..a], variable)?
                        - calculate(&text[a + 1..end], variable)?);
                } else {
                    return Ok(calculate(&text[start..a], variable)?
                        + calculate(&text[a + 1..end], variable)?);
                }
            }
        }
        bracket_level = 0;
        for a in (start..end).rev() {
            let i = a as usize; //what if i equals end???????? BUg
            if &text[i..i + 1] == ")" {
                bracket_level += 1;
            }
            if &text[i..i + 1] == "(" {
                bracket_level -= 1;
            }
            if (&text[i..i + 1] == "*" || &text[i..i + 1] == "/") && bracket_level == 0 {
                if &text[i..i + 1] == "*" {
                    return Ok(calculate(&text[start..a], variable)?
                        * calculate(&text[a + 1..end], variable)?);
                } else {
                    return Ok(calculate(&text[start..a], variable)?
                        / calculate(&text[a + 1..end], variable)?);
                }
            }
        }
        bracket_level = 0;
        for a in (start..end).rev() {
            let i: usize = a;

            if &text[i..i + 1] == ")" {
                bracket_level += 1;
            }
            if &text[i..i + 1] == "(" {
                bracket_level -= 1;
            }
            if &text[i..i + 1] == "^" && bracket_level == 0 {
                return Ok(calculate(&text[start..a], variable)?
                    .powf(calculate(&text[a + 1..end], variable)?));
            }
        }
        for a in (start..end).rev() {
            let i: usize = a;

            if &text[i..i + 1] == ")" {
                bracket_level += 1;
            }
            if &text[i..i + 1] == "(" {
                bracket_level -= 1;
            }
            if (&text[i..i + 1] == "s"
                || &text[i..i + 1] == "c"
                || &text[i..i + 1] == "t"
                || &text[i..i + 1] == "q")
                && bracket_level == 0
            {
                if &text[i..i + 1] == "s" {
                    return Ok(calculate(&text[i + 1..], variable)?.sin());
                } else {
                    if &text[i..i + 1] == "c" {
                        return Ok(calculate(&text[i + 1..], variable)?.cos());
                    } else {
                        if &text[i..i + 1] == "t" {
                            return Ok(calculate(&text[i + 1..], variable)?.sin()
                                / calculate(&text[i + 3..], variable)?.cos());
                        } else {
                            //sqrt
                            dbg!("test");
                            return Ok(calculate(&text[i + 1..], variable)?.powf(0.5));
                        }
                    }
                }
            }
        }

        //dbg!(&text[start..start + 1], &text[end - 1..]);
        if &text[start..start + 1] == "(" && &text[end - 1..] == ")" {
            return Ok(calculate(&text[start + 1..end - 1], variable)?);
        }
        bracket_level = 0;
        for a in start..end {
            if &text[a..a + 1] == ")" {
                bracket_level += 1;
            }
            if &text[a..a + 1] == "(" {
                bracket_level -= 1;
            }
        }
        if bracket_level == 0 {
            for i in start..end {
                for k in variable {
                    if &text[i..i + 1] == k.name {
                        return Ok(k.value);
                    }
                }
            }
            let d: f64 = equation.parse()?;
            return Ok(d as f64);
        } else {
            return Err(anyhow!("Mismatched paranthesis"));
        }
    }
    return Err(anyhow!("Nothing typed"));
}

fn trim_calc(text_trim: String) -> String {
    let untrimmed = text_trim.trim().to_lowercase();
    let mut refined: String = "".to_string();
    for c in untrimmed.chars() {
        match c {
            's' => refined.push_str("Sin"),
            'c' => refined.push_str("Cos"),
            't' => refined.push_str("Tan"),
            'q' => refined.push_str("√"),
            'p' => refined.push_str("π"),
            _ => refined.push(c),
        }
    }
    return refined;
}

// fn add_var(var: Vec<Variable>)->Vec<Variable>{
//     let mut mut_var= var;
//     let var_names = vec!['a','b','c','d','e','f'];
//     let new_var = Variable {
//         name: var_names[mut_var.len()],
//         value: 0,
//     };
//     mut_var.push(new_var);
//     return mut_var
// }
