use svg::{node::element::Rectangle, Document};

const COLOR: &str = "#198acd";

fn rect(x: f64, y: f64, width: f64, height: f64) -> Rectangle {
    Rectangle::new()
        .set("x", x)
        .set("y", y)
        .set("width", width)
        .set("height", height)
        .set("fill", COLOR)
        .set("stroke", "none")
}

/// Makes four rectangles on the corners of the given rectangle
/// at half the size (t-square fractal).
fn recurse(rectangle: &Rectangle) -> [Rectangle; 4] {
    let x = rectangle
        .get_attributes()
        .get("x")
        .unwrap()
        .parse::<f64>()
        .unwrap();
    let y = rectangle
        .get_attributes()
        .get("y")
        .unwrap()
        .parse::<f64>()
        .unwrap();
    let width = rectangle
        .get_attributes()
        .get("width")
        .unwrap()
        .parse::<f64>()
        .unwrap();
    let height = rectangle
        .get_attributes()
        .get("height")
        .unwrap()
        .parse::<f64>()
        .unwrap();

    let half_width = width / 2.0;
    let half_height = height / 2.0;

    [
        rect(
            x - half_width / 2.0,
            y - half_height / 2.0,
            half_width,
            half_height,
        ),
        rect(
            x + half_width * 3.0 / 2.0,
            y - half_height / 2.0,
            half_width,
            half_height,
        ),
        rect(
            x - half_width / 2.0,
            y + half_height * 3.0 / 2.0,
            half_width,
            half_height,
        ),
        rect(
            x + half_width * 3.0 / 2.0,
            y + half_height * 3.0 / 2.0,
            half_width,
            half_height,
        ),
    ]
}

fn add_rectangles(document: Document, rectangle: &Rectangle, depth: usize) -> Document {
    if depth == 0 {
        return document;
    }

    let rectangles = recurse(rectangle);
    let mut document = document;
    for rect in rectangles.iter() {
        document = document.add(rect.clone());
        document = add_rectangles(document, rect, depth - 1);
    }
    document
}

fn main() {
    const WIDTH: f64 = 100.0;
    const HEIGHT: f64 = 100.0;

    let main_rectangle = rect(WIDTH / 4.0, HEIGHT / 4.0, WIDTH / 2.0, HEIGHT / 2.0);

    let document = Document::new()
        .set("viewBox", (0, 0, 100, 100))
        .add(main_rectangle.clone());

    let document = add_rectangles(document.clone(), &main_rectangle, 6);

    svg::save("t-square.svg", &document).unwrap();
}
