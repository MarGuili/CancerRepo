use plotters::prelude::*;

// Data structure to represent patient information
#[derive(Debug)]
struct Patient {
    name: String,
    age: u32,
    has_cancer: bool,
}

impl Patient {
    fn new(name: &str, age: u32, has_cancer: bool) -> Self {
        Self {
            name: name.to_string(),
            age,
            has_cancer,
        }
    }
}

// Function to analyze and visualize patient data
fn analyze_and_visualize(data: &[Patient]) {
    let root = BitMapBackend::new("cancer_analysis.png", (800, 600)).into_drawing_area();
    root.fill(&WHITE).unwrap();

    let mut chart = ChartBuilder::on(&root)
        .caption("Cancer Analysis", ("Arial", 30).into_font())
        .margin(5)
        .x_label_area_size(30)
        .y_label_area_size(30)
        .build_cartesian_2d(0..100, 0..100)
        .unwrap();

    chart
        .configure_mesh()
        .disable_x_mesh()
        .disable_y_mesh()
        .draw()
        .unwrap();

    // Plot patient data
    for patient in data {
        let color = if patient.has_cancer {
            RGBColor(255, 0, 0) 
            RGBColor(0, 0, 255) 
        };

        chart.draw_series(PointSeries::of_element(
            vec![(patient.age as i32, 50)],
            5,
            &color,
            &|c, s, t| {
                return EmptyElement::at(c)    
                    + Circle::new((0, 0), s, t.filled());
            },
        ))
        .unwrap()
        .label(&patient.name);
    }

    chart.configure_series_labels().draw().unwrap();
}

fn main() {
    // Sample patient data
    let patients = vec![
        Patient::new("John", 45, true),
        Patient::new("Jane", 30, false),
        // Add more patient data as needed
    ];

    analyze_and_visualize(&patients);
}
