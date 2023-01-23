use crate::dbscan::model::Model;
use crate::dbscan::para_model::{Cell, ParallelModel};
use plotters::prelude::*;
use rand::Rng;

pub fn plot_model_2d(model: Model)
{
    let binding = model.get_dataset();
    let dataset = binding.lock().unwrap();
    let labels = dataset.get_labels();
    let data = dataset.get_data();
    let mut rng = rand::thread_rng();
    for (j, cluster) in model.get_clusters().iter().enumerate() {
        println!("{:?}", cluster);
        let (cluster, col_a, col_b) = cluster;
        let s = &format!("./plots/{}.png", j)[..];
        let root = BitMapBackend::new(s, (1290, 960)).into_drawing_area();
        root.fill(&WHITE).unwrap();
        let root = root.margin(10, 10, 10, 10);
        // After this point, we should be able to draw construct a chart context
        let (min1, max1) = find_min_max(cluster, *col_a, data);
        let (min2, max2) = find_min_max(cluster, *col_b, data);
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
                to_draw.push((data[*col_a][i] as f64, data[*col_b][i] as f64));
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
        println!("pp");
        root.present().unwrap();
    }
}

fn find_min_max(clusters: &Vec<Vec<usize>>, col: usize, data: &[Vec<f64>]) -> (f64, f64)
{
    let mut min = data[col][0];
    let mut max = data[col][0];
    for cluster in clusters {
        for &index in cluster {
            min = min.min(data[col][index]);
            max = max.max(data[col][index])
        }
    }

    (min, max)
}

pub fn plot_parallel_model_2d(model: ParallelModel)
{
    let pairs = model.get_pairs();
    let binding = model.get_dataset();
    let dataset = binding.lock().unwrap();
    let labels = dataset.get_labels();
    let focus = model.get_focus();
    let mut rng = rand::thread_rng();
    let clusters = model.get_clusters();
    // println!("{:?}", cluster);
    let (col_a, col_b) = (focus.0, focus.1);
    let s = &format!("./plots/{}.png", 0)[..];
    let root = BitMapBackend::new(s, (1290, 960)).into_drawing_area();
    root.fill(&WHITE).unwrap();
    let root = root.margin(10, 10, 10, 10);
    // After this point, we should be able to draw construct a chart context
    let ((min1, min2), (max1, max2)) = find_min_max_pairs(pairs);
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
        .x_desc(labels[col_a].clone())
        .x_label_style(("sans-serif", 20).into_font())
        .y_labels(5)
        .y_desc(labels[col_b].clone())
        .y_label_style(("sans-serif", 20).into_font())
        // We can also change the format of the label text
        .y_label_formatter(&|x| format!("{:.3}", x))
        .draw()
        .unwrap();

    // println!("{:?}", clusters);

    let mut to_draw: Vec<Vec<(f64, f64)>> =
        vec![vec![]; *clusters.iter().max().unwrap() as usize + 1];
    // let mut to_draw: Vec<Vec<(f64, f64)>> = vec![vec![]; pairs.len()];

    for i in 0..pairs.len() {
        // println!("{}, {}",clusters[i],clusters.iter().max().unwrap());
        if clusters[i] != -1 {
            to_draw[clusters[i] as usize].push((pairs[i].0 as f64, pairs[i].1 as f64))
            // to_draw[i].push((pairs[i].0 as f64, pairs[i].1 as f64));
        }
    }
    println!("pppp");
    for to_draw in to_draw {
        if to_draw.len() > model.get_min_pts() {
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
    }
    // for cluster in 0..*clusters.iter().max().unwrap()+1 {
    //     // println!("cluster: {}", cluster);
    //     let mut to_draw: Vec<(f64, f64)> = vec![];
    //     for i in 0..pairs.len() {
    //         if clusters[i] == cluster {
    //             to_draw.push((pairs[i].0 as f64, pairs[i].1 as f64));
    //         }
    //     }
    //     chart
    //         .draw_series(PointSeries::of_element(
    //             to_draw,
    //             5,
    //             RGBColor(rng.gen::<u8>(), rng.gen::<u8>(), rng.gen::<u8>()).filled(),
    //             &|c, s, st| {
    //                 EmptyElement::at(c)    // We want to construct a composed element on-the-fly
    //                     + Circle::new((0, 0), s, st.filled()) // At this point, the new pixel coordinate is established;
    //             },
    //         ))
    //         .unwrap();
    // }
    root.present().unwrap();
}

fn find_min_max_pairs(pairs: &Vec<(f64, f64)>) -> ((f64, f64), (f64, f64))
{
    let (mut min_x, mut min_y) = pairs[0];
    let (mut max_x, mut max_y) = pairs[0];
    for pair in pairs {
        min_x = min_x.min(pair.0);
        min_y = min_y.min(pair.1);
        max_x = max_x.max(pair.0);
        max_y = max_y.max(pair.1);
    }

    ((min_x, min_y), (max_x, max_y))
}

pub fn draw_cells(model: &ParallelModel)
{
    let pairs = model.get_pairs();
    let binding = model.get_dataset();
    let dataset = binding.lock().unwrap();
    let cells = model.get_cells();
    let labels = dataset.get_labels();
    let focus = model.get_focus();
    let mut rng = rand::thread_rng();
    let (col_a, col_b) = (focus.0, focus.1);
    let s = &format!("./plots/cells{}.png", 0)[..];
    let root = BitMapBackend::new(s, (1290 * 5, 960 * 5)).into_drawing_area();
    root.fill(&WHITE).unwrap();
    let root = root.margin(10, 10, 10, 10);
    // After this point, we should be able to draw construct a chart context
    let ((min1, min2), (max1, max2)) = find_min_max_pairs(pairs);
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
        .x_desc(labels[col_a].clone())
        .x_label_style(("sans-serif", 20).into_font())
        .y_labels(5)
        .y_desc(labels[col_b].clone())
        .y_label_style(("sans-serif", 20).into_font())
        // We can also change the format of the label text
        .y_label_formatter(&|x| format!("{:.3}", x))
        .draw()
        .unwrap();

    // println!("{:?}", clusters);

    let mut to_draw: Vec<Vec<(f64, f64)>> = vec![vec![]; cells.len() as usize];
    // let mut to_draw: Vec<Vec<(f64, f64)>> = vec![vec![]; pairs.len()];
    let mut frames_to_draw: Vec<Vec<(f64, f64)>> = vec![vec![]; cells.len() as usize];
    for (n, cell) in cells.iter().enumerate() {
        let (mut min_x, mut max_x, mut min_y, mut max_y) = (f64::MAX, f64::MIN, f64::MAX, f64::MIN);
        // println!("{}, {}",clusters[i],clusters.iter().max().unwrap());
        for i in cell.get_point_id()..cell.get_point_id() + cell.get_amount() {
            to_draw[n].push((pairs[i].0 as f64, pairs[i].1 as f64));
            min_x = min_x.min(pairs[i].0);
            max_x = max_x.max(pairs[i].0);
            min_y = min_y.min(pairs[i].1);
            max_y = max_y.max(pairs[i].1);
            // to_draw[i].push((pairs[i].0 as f64, pairs[i].1 as f64));
        }
        frames_to_draw[n].push((min_x, min_y));
        frames_to_draw[n].push((max_x, min_y));
        frames_to_draw[n].push((max_x, max_y));
        frames_to_draw[n].push((min_x, max_y));
        frames_to_draw[n].push((min_x, min_y));
    }
    println!("pppp");
    for (i, frame) in frames_to_draw.into_iter().enumerate() {
        if to_draw[i].len() < model.get_min_pts() {
            continue;
        }
        chart
            .draw_series(LineSeries::new(frame, &RGBColor(0, 0, 0)))
            .unwrap();
    }
    for to_draw in to_draw {
        if to_draw.len() > model.get_min_pts() {
            chart
                .draw_series(PointSeries::of_element(
                    to_draw,
                    2,
                    RGBColor(rng.gen::<u8>(), rng.gen::<u8>(), rng.gen::<u8>()).filled(),
                    &|c, s, st| {
                        EmptyElement::at(c)    // We want to construct a composed element on-the-fly
                            + Circle::new((0, 0), s, st.filled()) // At this point, the new pixel coordinate is established;
                    },
                ))
                .unwrap();
        }
    }
    // for cluster in 0..*clusters.iter().max().unwrap()+1 {
    //     // println!("cluster: {}", cluster);
    //     let mut to_draw: Vec<(f64, f64)> = vec![];
    //     for i in 0..pairs.len() {
    //         if clusters[i] == cluster {
    //             to_draw.push((pairs[i].0 as f64, pairs[i].1 as f64));
    //         }
    //     }
    //     chart
    //         .draw_series(PointSeries::of_element(
    //             to_draw,
    //             5,
    //             RGBColor(rng.gen::<u8>(), rng.gen::<u8>(), rng.gen::<u8>()).filled(),
    //             &|c, s, st| {
    //                 EmptyElement::at(c)    // We want to construct a composed element on-the-fly
    //                     + Circle::new((0, 0), s, st.filled()) // At this point, the new pixel coordinate is established;
    //             },
    //         ))
    //         .unwrap();
    // }
    root.present().unwrap();
}
