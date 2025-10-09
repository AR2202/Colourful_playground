#[derive(Clone, Debug, PartialEq, Eq, Hash)]

pub enum PrimaryColor {
    Red,
    Yellow,
    Blue,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]

pub enum Color {
    Primary(PrimaryColor),
    Orange,
    Green,
    Purple,
    Pink,
    Cyan,
    Violet,
    Lime,
    Teal,
    // etc.
}
impl std::fmt::Display for Color {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Color::Primary(PrimaryColor::Red) => "red",
            Color::Primary(PrimaryColor::Blue) => "blue",
            Color::Primary(PrimaryColor::Yellow) => "yellow",
            Color::Orange => "orange",
            Color::Green => "green",
            Color::Purple => "purple",
            Color::Pink => "pink",
            Color::Cyan => "cyan",
            Color::Violet => "violet",
            Color::Lime => "lime",
            Color::Teal => "teal",
        };
        write!(f, "{}", s)
    }
}

#[derive(Debug, Clone)]
pub enum Expr {
    Prim(PrimaryColor),
    Apply(Box<Expr>, Box<Expr>),
}

fn apply_chain(exprs: Vec<Expr>) -> Expr {
    exprs
        .into_iter()
        .reduce(|acc, next| Expr::Apply(Box::new(acc), Box::new(next)))
        .unwrap()
}
pub fn normalize_color(color: &Color) -> Expr {
    use PrimaryColor::*;
    match color {
        Color::Primary(p) => Expr::Prim(p.clone()),
        Color::Orange => apply_chain(vec![Expr::Prim(Red), Expr::Prim(Yellow)]),
        Color::Green => apply_chain(vec![Expr::Prim(Blue), Expr::Prim(Yellow)]),
        Color::Purple => apply_chain(vec![Expr::Prim(Red), Expr::Prim(Blue)]),
        Color::Pink => apply_chain(vec![Expr::Prim(Red), Expr::Prim(Red)]),
        Color::Cyan => apply_chain(vec![Expr::Prim(Blue), Expr::Prim(Blue)]),
        Color::Violet => apply_chain(vec![Expr::Prim(Blue), Expr::Prim(Red)]),
        Color::Lime => apply_chain(vec![
            Expr::Prim(Blue),
            Expr::Prim(Yellow),
            Expr::Prim(Yellow),
        ]),
        Color::Teal => apply_chain(vec![Expr::Prim(Blue), Expr::Prim(Yellow), Expr::Prim(Blue)]),
    }
}
/// Build an expression from a right-to-left list of Colors
pub fn build_expr_from_colors(colors: &[Color]) -> Expr {
    let reversed = colors.iter().rev();
    let mut iter = reversed.map(normalize_color);

    let first = iter.next().unwrap();

    iter.fold(first, |acc, expr| {
        Expr::Apply(Box::new(acc), Box::new(expr))
    })
}

/// Evaluate the expression using SKI-style rules
pub fn evaluate(expr: Expr) -> Expr {
    match expr.clone() {
        Expr::Apply(f, x) => match *f {
            Expr::Prim(PrimaryColor::Yellow) => evaluate(*x),

            Expr::Apply(f2, x2) => match *f2.clone() {
                Expr::Prim(PrimaryColor::Red) => evaluate(*x2),
                Expr::Prim(PrimaryColor::Blue) => expr.clone(),
                Expr::Prim(PrimaryColor::Yellow) => {
                    evaluate(Expr::Apply(Box::new(*x2), Box::new(*x.clone())))
                }

                Expr::Apply(f3, x3) => match *f3 {
                    Expr::Prim(PrimaryColor::Blue) => {
                        let fx = Expr::Apply(Box::new(*x3), Box::new(*x.clone()));
                        let gx = Expr::Apply(Box::new(*x2), Box::new(*x.clone()));
                        evaluate(Expr::Apply(Box::new(fx), Box::new(gx)))
                    }

                    other => {
                        let left = evaluate(Expr::Apply(Box::new(*f2), Box::new(*x2)));
                        let right = evaluate(*x);
                        evaluate(Expr::Apply(Box::new(left),  Box::new(right)))
                    }
                },
            },

            other => {
                let left = other;
                let right = evaluate(*x);
                Expr::Apply(Box::new(left.clone()), Box::new(right))
            }
        },

        Expr::Prim(_) => expr.clone(),
    }
}

/// Convert an expression back to Colourful syntax
pub fn to_colourful(expr: &Expr) -> String {
    match expr {
        Expr::Prim(c) => match c {
            PrimaryColor::Red => "red".to_string(),
            PrimaryColor::Blue => "blue".to_string(),
            PrimaryColor::Yellow => "yellow".to_string(),
        },
        Expr::Apply(f, x) => format!("({} {})", to_colourful(f), to_colourful(x)),
    }
}
pub fn extract_colors(expr: &Expr) -> Vec<Color> {
    match expr {
        Expr::Prim(p) => vec![Color::Primary(p.clone())],

        Expr::Apply(f, x) => match (&**f, &**x) {
            // Teal: ((Blue Yellow) Blue)
            (Expr::Apply(f1, x1), Expr::Prim(PrimaryColor::Blue))
                if matches!(
                    (&**f1, &**x1),
                    (
                        Expr::Prim(PrimaryColor::Blue),
                        Expr::Prim(PrimaryColor::Yellow)
                    )
                ) =>
            {
                vec![Color::Teal]
            }

            // Lime: ((Blue Yellow) Yellow)
            (Expr::Apply(f1, x1), Expr::Prim(PrimaryColor::Yellow))
                if matches!(
                    (&**f1, &**x1),
                    (
                        Expr::Prim(PrimaryColor::Blue),
                        Expr::Prim(PrimaryColor::Yellow)
                    )
                ) =>
            {
                vec![Color::Lime]
            }

            // Other known 2-color combinations
            (Expr::Prim(PrimaryColor::Red), Expr::Prim(PrimaryColor::Yellow)) => {
                vec![Color::Orange]
            }
            (Expr::Prim(PrimaryColor::Blue), Expr::Prim(PrimaryColor::Yellow)) => {
                vec![Color::Green]
            }
            (Expr::Prim(PrimaryColor::Red), Expr::Prim(PrimaryColor::Blue)) => vec![Color::Purple],
            (Expr::Prim(PrimaryColor::Red), Expr::Prim(PrimaryColor::Red)) => vec![Color::Pink],
            (Expr::Prim(PrimaryColor::Blue), Expr::Prim(PrimaryColor::Blue)) => vec![Color::Cyan],
            (Expr::Prim(PrimaryColor::Blue), Expr::Prim(PrimaryColor::Red)) => vec![Color::Violet],

            // Fallback: recursively flatten
            _ => {
                let mut left = extract_colors(f);
                let mut right = extract_colors(x);
                left.append(&mut right);
                left
            }
        },
    }
}
