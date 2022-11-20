use crate::dbscan::model::Model;
use plotters::prelude::*;
use rand::Rng;

pub fn plot_model_2d(model: Model) {
    let dataset = model.get_dataset().get_data();
    let labels = model.get_dataset().get_labels();
    let mut rng = rand::thread_rng();
    for (j, cluster) in model.get_clusters().iter().enumerate() {
        let (cluster, col_a, col_b) = cluster;
        let s = &format!("./plots/{}.png", j)[..];
        let root = BitMapBackend::new(s, (1290, 960)).into_drawing_area();
        root.fill(&WHITE).unwrap();
        let root = root.margin(10, 10, 10, 10);
        // After this point, we should be able to draw construct a chart context
        let (min1, max1) = find_min_max(cluster, *col_a, dataset);
        let (min2, max2) = find_min_max(cluster, *col_b, dataset);
        let (kx, ky) = (max1 - min1, max2 - min2);
        let mut chart = ChartBuilder::on(&root)
            // Set the caption of the chart
            .caption("DBSCAN test", ("sans-serif", 40).into_font())
            // Set the size of the label region
            .x_label_area_size(40)
            .y_label_area_size(60)
            // Finally attach a coordinate on the drawing area and make a chart context
            .build_cartesian_2d(
                min1 - kx * 0.1..max1 + kx * 0.1,
                min2 - ky * 0.1..max2 + ky * 0.1,
            )
            .unwrap();

        // Then we can draw a mesh
        chart
            .configure_mesh()
            // We can customize the maximum number of labels allowed for each axis
            .x_labels(5)
            .x_desc(labels[*col_a].clone())
            .x_label_style(("sans-serif", 20).into_font())
            .y_labels(5)
            .y_desc(labels[*col_b].clone())
            .y_label_style(("sans-serif", 20).into_font())
            // We can also change the format of the label text
            .y_label_formatter(&|x| format!("{:.3}", x))
            .draw()
            .unwrap();

        for cluster in cluster {
            let mut to_draw: Vec<(f64, f64)> = vec![];
            for &i in cluster {
                to_draw.push((dataset[*col_a][i] as f64, dataset[*col_b][i] as f64));
            }
            chart
                .draw_series(PointSeries::of_element(
                    to_draw,
                    5,
                    RGBColor(rng.gen::<u8>(), rng.gen::<u8>(), rng.gen::<u8>()).filled(),
                    &|c, s, st| {
                        EmptyElement::at(c)    // We want to construct a composed element on-the-fly
                        + Circle::new((0, 0), s, st.filled()) // At this point, the new pixel coordinate is established;
                    },
                ))
                .unwrap();
        }
        root.present().unwrap();
    }
}

fn find_min_max(clusters: &Vec<Vec<usize>>, col: usize, dataset: &[Vec<f64>]) -> (f64, f64) {
    let mut min = dataset[col][0];
    let mut max = dataset[col][0];
    for cluster in clusters {
        for &index in cluster {
            min = min.min(dataset[col][index]);
            max = max.max(dataset[col][index])
        }
    }

    (min, max)
}
