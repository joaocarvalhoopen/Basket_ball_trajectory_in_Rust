/// Name: Basketball trajectory in Rust
/// 
/// Author: João Nuno Carvalho
/// Date:   2022.01.06
/// Description: This tries to answer the question if the ball that a basket
///              player throws with a V_0 velocity vector and a Teta angle
///              will enter the basket in a parabolic trajectory?
///              I made this to illustrate to my daughter that the equations
///              in her physics book could came out "alive" if they were
///              calculated with a simple program. I applied to basket, a
///              game that my daughter likes. It calculates for each instant t
///              and in the end it draws the trajectory of the ball in a
///              SVG animation and in text mode.
///
/// License: MIT Open Source License.
/// 
/// Equations in 2D:
/// 
///    Uniformed accelerated movement:          
///              s = s_0 + v_0 * t - 1/2 * g * t^2
/// 
///    Decomposed movement into is components XX and YY:
///              v_0_x = v_0 * cos(teta_0)
///              v_0_y = v_0 * sin(teta_0)
/// 
///              ball_pos_x = x_0 + v_0_x * t
///              ball_pos_y = y_0 + v_0_y * t - 1/2 * GRAVITY * t^2 
///
///    Euclidean distance 3D:
///              dist = sqrt( (p_x - q_x)^2 + (p_y - q_y)^2 + (p_z - q_z)^2 )
///   
/// References: 
///    Projectile motion
///    https://en.wikipedia.org/wiki/Projectile_motion


mod svg_gen;

use crate::svg_gen::Color;

const GRAVITY: f64 = 9.807; // m / s^2 - Meters per second square.
const MIN_BALL_DELTA_TO_BASKET_CENTER: f64 = 0.1; // 10 cm

type Trajectory = (bool, Vec<(f64, (f64, f64), bool)>);

fn main() {
    println!("********************************************");
    println!("** Did the basketball go into the basket? **");
    println!("********************************************");
    
    // Player throw position.
    let pos_0_x: f64 = 0.0;   // m - meters
    let pos_0_y: f64 = 1.5;   // m - meters 
    let pos_0_z: f64 = 0.0;   // m - meters
    
    // Initial velocity vector.
    let v_0:    f64 = 10.0;   // m/s - Meters per second
    let teta_0: f64 = 45.0;   // teta degrees =  angle in degrees XX axis to YY axis.
    let phi_0:  f64 =  0.0;   // phi  degrees =  angle in degrees ZZ axis to XX axis.  
    
    // Basket position.
    let basket_pos_x: f64 = 8.0;   // m - meters
    let basket_pos_y: f64 = 3.05;  // m - meters
    let basket_pos_z: f64 = 5.0;   // m - meters

    // Test the simulation for how many seconds?
    let simulation_sec: f64 = 3.0;  // s - Seconds to simulate.
    let num_steps: u32      = 60;   // Divide the simulation seconds into N equal points.

    let svg_trajectory_filename = "basketball_trajectory.svg";
    let svg_file_path = "./";
    let svg_x_max: f32 = 500.0;   // Max XX Coordinate.
    let svg_y_max: f32 = 300.0; // 500.0; //300.0;   // Max YY Coordinate.

    print_initial_data(pos_0_x, pos_0_y, pos_0_z, 
                       v_0, teta_0, phi_0,
                       basket_pos_x, basket_pos_y, basket_pos_z,
                       simulation_sec, num_steps,
                       svg_trajectory_filename);

    let num_rows = 50; // 80;
    let num_cols = 80;
    let rows_meters = 10.0; // m - meters
    let cols_meters = 10.0; // m - meters
    let mut display_cmd: DisplayCMD = DisplayCMD::new(num_rows, num_cols, rows_meters, cols_meters);

    let trajectory_2d = basketball_2d(pos_0_x, pos_0_y, 
                                      v_0, teta_0,
                                      basket_pos_x, basket_pos_y,
                                      simulation_sec, num_steps);

    print_trajectory_2d(& trajectory_2d, & mut display_cmd);

    let svg = plot_trajectory_svg(& trajectory_2d,
                                      basket_pos_x, basket_pos_y,
                                      svg_x_max, svg_y_max);
   
    // let file_str = svg.to_file_string();
    // println!("{}", file_str);

    let _ = svg.to_file(svg_trajectory_filename, svg_file_path);
}

fn conv_meters_sec_to_km_hour(vel: f64) -> f64 {
    (vel * 3_600.0) / 1_000.0 
}

fn print_initial_data(pos_0_x: f64, pos_0_y: f64, pos_0_z: f64, 
                      v_0: f64, teta_0: f64, phi_0: f64,
                      basket_pos_x: f64, basket_pos_y: f64, basket_pos_z: f64,
                      simulation_sec: f64, num_steps: u32,
                      svg_trajectory_filename: & str) {

    println!("Data:");
    
    println!("\n  Player throw position:");
    println!("    pos_0_x: {:0.2} m - meters", pos_0_x);
    println!("    pos_0_y: {:0.2} m - meters", pos_0_y);
    println!("    pos_0_z: {:0.2} m - meters", pos_0_z);
    
    println!("\n  Initial velocity vector:");
    println!("    v_0: {:0.2} m/s - Meters per second", v_0);
    println!("    v_0: {:0.2} Km/h - Km per hour", conv_meters_sec_to_km_hour(v_0));
    println!("    teta_0: {:0.2} degrees = angle in degrees XX axis to YY axis.", teta_0);
    println!("    phi_0: {:0.2} degrees = angle in degrees ZZ axis to XX axis.", phi_0);  
    
    println!("\n  Basket position:");
    println!("    basket_pos_x: {:0.2} m - meters", basket_pos_x);
    println!("    basket_pos_y: {:0.2} m - meters", basket_pos_y);
    println!("    basket_pos_z: {:0.2} m - meters", basket_pos_z);

    println!("\n  Test the simulation for how many seconds?");
    println!("    simulation_sec: {:0.2} s - Seconds to simulate", simulation_sec);
    println!("    num_steps: {:0.2}        - Divide the simulation seconds into N equal points.", num_steps );

    println!("\n  Output SVG");
    println!("    svg_trajectory_filename = {}", svg_trajectory_filename);
}

fn basketball_2d(pos_0_x: f64, pos_0_y: f64, 
                 v_0: f64, teta_0: f64,
                 basket_pos_x: f64, basket_pos_y: f64,
                 simulation_sec: f64, num_steps: u32)
                 -> Trajectory {
    
    // The velocity is positive and not zero.
    assert!(v_0 > 0.0);
    // We will simulate a non negative and a non zero time.
    assert!(simulation_sec > 0.0);
    // We will simulate at least 2 steps.
    assert!(num_steps > 2);

    let v_0_x = v_0 * f64::cos(teta_0);
    let v_0_y = v_0 * f64::sin(teta_0);

    let x_0 =  pos_0_x;
    let y_0 = pos_0_y;

    let time_steps = get_time_steps(simulation_sec, num_steps);

    let mut trajectory_2d: Vec<(f64, (f64, f64), bool)> = Vec::new();

    let mut flag_into_the_basket = false;

    for t in time_steps {
        let ball_x = x_0 + v_0_x * t;
        let ball_y = y_0 + v_0_y * t - (1.0/2.0) * GRAVITY * t * t; 
        let dist = euclidean_distance(
            ball_x, ball_y, 0.0,
             basket_pos_x, basket_pos_y, 0.0);
        let mut flag_enter_instant = false;
        if dist <= MIN_BALL_DELTA_TO_BASKET_CENTER {
            flag_into_the_basket = true;
            flag_enter_instant = true;
        }
        if ball_y >= 0.0 {
            trajectory_2d.push( (t, (ball_x, ball_y), flag_enter_instant) );
        }
    }
    (flag_into_the_basket, trajectory_2d)
}

/*
fn basketball_3d(pos_0_x: f64, pos_0_y: f64, pos_0_z: f64, 
                 v_0: f64, teta_0: f64, phi_0: f64,
                 basket_pos_x: f64, basket_pos_y: f64, basket_pos_z: f64,
                 simulation_sec: f64, num_steps: u32)
                 -> (bool, Vec<(f64, (f64, f64, f64))) {

}
*/

fn get_time_steps(simulation_sec: f64, num_steps: u32) -> Vec<f64> {
    let inner_steps = num_steps - 1;
    let delta_t = simulation_sec / inner_steps as f64;
    let mut time_steps_vec: Vec<f64> = Vec::new();
    // Add the first instant 0.0 s.
    time_steps_vec.push(0.0);
    for step in 1..(inner_steps + 1) {
        time_steps_vec.push(delta_t * step as f64);    
    }
    // Add the last instant simulation_sec s.
    time_steps_vec.push(simulation_sec);
    time_steps_vec
}

fn euclidean_distance(p_x: f64, p_y: f64, p_z: f64,
                      q_x: f64, q_y: f64, q_z: f64)
                      -> f64 {
    f64::sqrt((p_x - q_x).powi(2) + (p_y - q_y).powi(2) + (p_z - q_z).powi(2))
}

fn print_trajectory_2d(trajectory_2d: & Trajectory, display_cmd: & mut DisplayCMD) {
    println!("\n****************");
    println!("** Trajectory **");
    println!("****************");
    println!("  Entered the basket: {}", trajectory_2d.0);
    println!("");

    for (t, (x, y), flag_enter_instant) in & trajectory_2d.1 {
        println!("  t: {:0.2} s, x: {:0.2} m, y: {:0.2} m, {} ", t, x, y, if *flag_enter_instant {"ball entered the basket"} else {""} );
        display_cmd.set_pixel_meters('O', *y, *x, *flag_enter_instant);
    }
    println!("");

    display_cmd.print();
}

struct DisplayCMD {
    buf: Vec<char>,
    num_rows: usize,
    num_cols: usize,
    rows_meters: f64,
    cols_meters: f64,
}

impl DisplayCMD {
    fn new(num_rows: usize, num_cols: usize, rows_meters: f64, cols_meters: f64) -> Self {
        DisplayCMD { 
            buf: vec![' '; num_rows * num_cols],
            num_rows,
            num_cols,
            rows_meters,
            cols_meters, 
        }
    }

    fn set_pixel(& mut self, ch: char, row: usize, col: usize) {
        // println!("row: {}, col: {}", row, col);
        assert!(row < self.num_rows);
        assert!(col < self.num_cols);
        self.buf[row*self.num_cols + col] = ch;
    }

    fn set_pixel_meters(& mut self, ch: char, row_meters_p: f64, col_meters_p: f64, flag_enter_instant: bool) {
        assert!(row_meters_p <= self.rows_meters);
        assert!(col_meters_p <= self.cols_meters);
        let row = (row_meters_p * (self.num_rows - 1) as f64) / self.rows_meters;
        let col = (col_meters_p * (self.num_cols - 1) as f64) / self.cols_meters;
        let row = f64::round(row) as usize;
        let col = f64::round(col) as usize;
        let mut ch = ch;
        if flag_enter_instant {
            ch = '=';
            self.set_pixel(ch, row, col - 2);
            self.set_pixel(ch, row, col - 1);
            self.set_pixel(ch, row, col + 1);
            self.set_pixel(ch, row, col + 2);
            ch = '*';
        }
        self.set_pixel(ch, row, col);

    }

    fn get_pixel(& self, row: usize, col: usize) -> char {
        assert!(row < self.num_rows);
        assert!(col < self.num_cols);
        self.buf[row*self.num_cols + col]
    }

    fn print(& self) {
        for row in (0..self.num_rows).rev() {
            for col in 0..self.num_cols {
                print!("{}", self.get_pixel(row, col));
            }
            println!("");
        }
    }
}

fn plot_trajectory_svg(trajectory_2d: & Trajectory,
                       basket_pos_x: f64, basket_pos_y: f64,
                       svg_x_max: f32, svg_y_max: f32 ) -> svg_gen::SVG {

    debug_assert!(svg_x_max > 0.0);
    debug_assert!(svg_y_max > 0.0);

    use std::fmt::Write;

    let mut svg = svg_gen::SVG::new(svg_x_max, svg_y_max, Some(Color::Black));

    // NOTE: Copied the SVG file output value to sublime, selected the text and see the number
    //       of bytes, single byte characters.
    const FINAL_SVG_TEXT_SIZE: usize = 10_000;
    let mut elem_str = String::with_capacity(FINAL_SVG_TEXT_SIZE);

    let mut x_max: f64 = f64::MIN;
    let mut y_max: f64 = f64::MIN;
    // Find x_max and y_max in the trajectory.
    for (t, (x, y), flag_enter_instant) in & trajectory_2d.1 {    
        if *x > x_max {
            x_max = *x;
        }
        if *y > y_max {
            y_max = *y;
        }
    }
    let max_x_y = f64::max(x_max, y_max);
    let scale_factor = svg_x_max as f64 / max_x_y;

    /*
        <circle id="circle" cx="0" cy="0" r="3" fill="yellow" />
      
        <animateMotion
                xlink:href="#circle"
                dur="3s"
                begin="0s"
                fill="freeze"
                repeatCount="indefinite">
            <mpath xlink:href="#motionPath" />
        </animateMotion>
    */

    for (t, (x, y), flag_enter_instant) in & trajectory_2d.1 {    
        // Draw the circle.
        // <circle cx="150" cy="100" r="2" fill="blue" />
        let _ = write!(elem_str, 
                "<circle cx=\"{0:.2}\" cy=\"{1:.2}\" r=\"{2:.2}\" fill=\"{3}\" />\n",
                x * scale_factor,
                svg_y_max as f64 - y * scale_factor,
                2.0,
                if *flag_enter_instant {"green"} else {"blue"}
            );
    }

    // Draw the basket.
    // "<rect x="100" y="200" width="20" height="5" style="fill:green;stroke:green;stroke-width:1.0" />\n",
    let _ = write!(elem_str,
              "<rect x=\"{0:.2}\" y=\"{1:.2}\" width=\"{2:.2}\" height=\"{3:.2}\" style=\"fill:green;stroke:green;stroke-width:{4:.2}\" />\n",
              basket_pos_x * scale_factor - 10.0,
              svg_y_max as f64 - basket_pos_y * scale_factor - 2.0,
              20.0,
              4.0,
              1.0);

    // Get the position zero of the trajectory of the basket ball.
    let x_0 = trajectory_2d.1[0].1.0 * scale_factor; 
    let y_0 = svg_y_max as f64 - trajectory_2d.1[0].1.1 * scale_factor;

    // Motion path.
    // <path id="motionPath" fill="none" stroke="#000000" d="M0,0L100,100L200,200" />
    let _ = write!(elem_str, 
            "<path id=\"motionPath\" fill=\"none\" d=\"M{0:.2},{1:.2}\n",
            x_0,
            y_0);

    let mut flag_skip_first = true;
    for (t, (x, y), flag_enter_instant) in & trajectory_2d.1 {    
        //if flag_skip_first {
        //    flag_skip_first = false;
        //    continue;
        //}
        // Draw the circle.
        // "L100,200\n"
        let _ = write!(elem_str, 
                // "L{0:.2},{1:.2}\n",
                // "L{0},{1}\n",
                "L{0:.2},{1:.2}\n",
                x * scale_factor,
                svg_y_max as f64 - y * scale_factor);

    }
    let _ = write!(elem_str, "\" />\n" );

    // "<circle id="circle" cx="%.2f" cy="%.2f" r="3" fill="yellow" />\n"
    let _ = write!(elem_str, 
        "<circle id=\"circle\" cx=\"{0:.2}\" cy=\"{1:.2}\" r=\"{2}\" fill=\"yellow\" />\n",
        0.0,
        0.0,
        3);

    /*
        <animateMotion
                xlink:href="#circle"
                dur="3s"
                begin="0s"
                fill="freeze"
                repeatCount="indefinite">
            <mpath xlink:href="#motionPath" />
        </animateMotion>
    */
    let _ = write!(elem_str,
            "<animateMotion
                xlink:href=\"#circle\"
                dur=\"3s\"
                begin=\"0s\"
                fill=\"freeze\"
                repeatCount=\"indefinite\">
                <mpath xlink:href=\"#motionPath\" />
            </animateMotion>"
            );

    svg.add_elem(elem_str);

    svg
}

