use std::{array, fmt, fs::File, io::{self, Write}, ops, process::ExitCode};
use json::Value;
use serde_json as json;


enum BoundaryCondition {
    Clip,
    Fit
}

#[derive(Clone)]
struct SpaceData {
    b: Field3Vec,
    e: Field3Vec,
    //neighbors: Neighbors<'a>
    object_index: usize
}
impl Default for SpaceData {
    fn default() -> Self {
        Self {
            e: Field3Vec::default(),
            b: Field3Vec::default(),
            object_index: 0
        }
    }
}
impl ops::Add<SpaceData> for SpaceData {
    type Output = SpaceData;

    fn add(self, rhs: SpaceData) -> Self::Output {
        Self { 
            e: self.e + rhs.e,
            b: self.b + rhs.b,
            ..Self::default()
        }
    }
}
impl ops::Sub<SpaceData> for SpaceData {
    type Output = SpaceData;

    fn sub(self, rhs: SpaceData) -> Self::Output {
        Self {
            e: self.e - rhs.e,
            b: self.b - rhs.b,
            ..Self::default()
        }
    }
}


#[derive(Clone)]
struct Field3Vec {
    components: Vec<f32>
}
impl Default for Field3Vec {
    fn default() -> Self {
        Self {components: vec![0.0, 0.0, 0.0]}
    }
}
impl ops::Add<Field3Vec> for Field3Vec {
    type Output = Field3Vec;

    fn add(self, rhs: Field3Vec) -> Self::Output {
        Self { components: vec![
                self.components[0] + rhs.components[0],
                self.components[1] + rhs.components[1],
                self.components[2] + rhs.components[2],
        ] }
    }
}
impl ops::Add<&Field3Vec> for &Field3Vec {
    type Output = Field3Vec;

    fn add(self, rhs: &Field3Vec) -> Self::Output {
        Field3Vec { components: vec![
                self.components[0] + rhs.components[0],
                self.components[1] + rhs.components[1],
                self.components[2] + rhs.components[2],
        ] }
    }
}
impl ops::Sub<Field3Vec> for Field3Vec {
    type Output = Field3Vec;

    fn sub(self, rhs: Field3Vec) -> Self::Output {
        Field3Vec { components: vec![
                self.components[0] - rhs.components[0],
                self.components[1] - rhs.components[1],
                self.components[2] - rhs.components[2],
        ] }
    }
}
impl ops::Sub<&Field3Vec> for &Field3Vec {
    type Output = Field3Vec;

    fn sub(self, rhs: &Field3Vec) -> Self::Output {
        Field3Vec { components: vec![
                self.components[0] - rhs.components[0],
                self.components[1] - rhs.components[1],
                self.components[2] - rhs.components[2],
        ] }
    }
}



impl ops::Mul<f32> for Field3Vec {
    type Output = Field3Vec;

    fn mul(self, rhs: f32) -> Self::Output {
        Self { components: vec![
                self.components[0] * rhs,
                self.components[1] * rhs,
                self.components[2] * rhs,
        ] }
    }
}
impl ops::Mul<f32> for &Field3Vec {
    type Output = Field3Vec;

    fn mul(self, rhs: f32) -> Self::Output {
        Field3Vec { components: vec![
                self.components[0] * rhs,
                self.components[1] * rhs,
                self.components[2] * rhs,
        ] }
    }
}
impl ops::Div<f32> for Field3Vec {
    type Output = Field3Vec;

    fn div(self, rhs: f32) -> Self::Output {
        Self { components: vec![
                self.components[0] / rhs,
                self.components[1] / rhs,
                self.components[2] / rhs,
        ] }
    }
}
impl ops::Div<f32> for &Field3Vec {
    type Output = Field3Vec;

    fn div(self, rhs: f32) -> Self::Output {
        Field3Vec { components: vec![
                self.components[0] / rhs,
                self.components[1] / rhs,
                self.components[2] / rhs,
        ] }
    }
}
impl fmt::Display for Field3Vec {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "<{}, {}, {}>", self.components[0], self.components[1], self.components[2])
    }
}

impl ops::Mul<Field3Vec> for f32 {
    type Output = Field3Vec;

    fn mul(self, rhs: Field3Vec) -> Self::Output {
        rhs.mul(self)
    }
}

trait CurrentObject {
    fn currrent_density(&self, t: f32) -> Field3Vec;
}


#[derive(Clone)]
struct Wire {
    angular_frequency: f32,
    amplitude: f32,
    direction: Field3Vec,
}
impl CurrentObject for Wire {
    fn currrent_density(&self, t: f32) -> Field3Vec {
        self.amplitude * (t * self.angular_frequency).sin() * self.direction.clone()
    }
}


#[derive(Clone)]
struct Vaccum;
impl CurrentObject for Vaccum {
    fn currrent_density(&self, t: f32) -> Field3Vec {
        Field3Vec::default()
    }
}

/*impl Field3Vec {
    fn dot(self, rhs: Self) -> f32 {
        self.components[0] * rhs.components[0] +
        self.components[1] * rhs.components[1] +
        self.components[2] * rhs.components[2]
    }

    fn cross(self, rhs: Self) -> Self {
        Self { components: vec![
            self.components[1] * self.components[2] - self.components[2] * self.components[1],
            self.components[2] * self.components[0] - self.components[0] * self.components[2],
            self.components[0] * self.components[1] - self.components[1] * self.components[0]
        ] }
    }
}*/


fn generate_blank_latice() -> LaticeType {
    Box::new(array::from_fn(
        |_| Box::new(array::from_fn(
            |_| Box::new(array::from_fn(
                |_| SpaceData::default()
            ))
        ))
    ))
}

fn update_progress_bar(val: u32, max: u32, message:&str) -> () {
    eprint!(
        "\r{} ({}/{}) ({:.1}%) [{: <10}]",
        message,
        val,
        max,
        100.0 * val as f32 / max as f32,
        format!("{:#<1$}", "", (10 * val / max) as usize)
    )
}




const LATICE_DENSITY: u32 = 30;
const SIMULATION_SIDE_LENGTH: u32 = 1;

//type LaticeType = [[[SpaceData; LATICE_DENSITY as usize*SIMULATION_SIDE_LENGTH as usize]; LATICE_DENSITY as usize*SIMULATION_SIDE_LENGTH as usize]; LATICE_DENSITY as usize*SIMULATION_SIDE_LENGTH as usize];
type LaticeType = Box<[Box<[Box<[SpaceData; LATICE_DENSITY as usize*SIMULATION_SIDE_LENGTH as usize]>; LATICE_DENSITY as usize*SIMULATION_SIDE_LENGTH as usize]>; LATICE_DENSITY as usize*SIMULATION_SIDE_LENGTH as usize]>;

fn main() -> ExitCode {
    eprint!("Manifest filename? ");
    let mut input = String::new();
    let _ = io::stdin().read_line(&mut input);
    let manifest_filename = &input[0..input.len()-2];

    eprintln!("");
    eprintln!("Simulating from \"{manifest_filename}\"...");
    
    //eprintln!("{manifest_filename}");
    //let a = manifest_filename.len();
    //eprintln!("{a}");
    let json_file = File::open(manifest_filename);
    if json_file.is_err() {
        return ExitCode::FAILURE;
    }
    let json_data: Value = json::from_reader(json_file.unwrap()).unwrap();


    let permittivity: f32 = json_data["constants"]["e0"].as_f64().unwrap() as f32;//8.85e-12;
    let permeability: f32 = json_data["constants"]["m0"].as_f64().unwrap() as f32;//PI*4e-7;
    let e0: f32 = permittivity;
    let m0: f32 = permeability;
    let dt: f32 = json_data["constants"]["dt"].as_f64().unwrap() as f32;
    let steps: u32 = json_data["constants"]["steps"].as_i64().unwrap() as u32;
    let time_culling_factor:u32 = json_data["constants"]["time_culling_factor"].as_i64().unwrap() as u32;
    let space_culling_factor:u32 = json_data["constants"]["space_culling_factor"].as_i64().unwrap() as u32;
    let boundary_condition:BoundaryCondition;
    if json_data["constants"]["boundary_condition"].as_str().unwrap().to_lowercase() == "clip" {
        boundary_condition = BoundaryCondition::Clip;
    } else if json_data["constants"]["boundary_condition"].as_str().unwrap().to_lowercase() == "fit" {
        boundary_condition = BoundaryCondition::Fit;
    } else {
        return ExitCode::FAILURE;
    }
    
    // Prepare pipeline to display program
    let mut out_writer = io::BufWriter::new(io::stdout());
    // Send constants for data reconstruction
    let _ = out_writer.write_all(&(LATICE_DENSITY as f32 / space_culling_factor as f32).to_le_bytes());
    let _ = out_writer.write_all(&(dt * time_culling_factor as f32).to_le_bytes());
    // Prepare memory for field data
    let mut current = generate_blank_latice();
    let mut next = generate_blank_latice();
    
    let mut field_objects: Vec<Box<dyn CurrentObject>> = vec![Box::new(Vaccum{})];

    let mut current_field_object_index:usize = 1;

    // Configure initial conditions
    for object in json_data["objects"].as_array().unwrap() {
        let mut field_object: Box<dyn CurrentObject>;
        let object_type = object["type"].as_str().unwrap();
        if object_type == "point" {
            let e = object["E"].as_array().unwrap();
            let e_x = e[0].as_f64().unwrap() as f32;
            let e_y = e[1].as_f64().unwrap() as f32;
            let e_z = e[2].as_f64().unwrap() as f32;

            let b = object["B"].as_array().unwrap();
            let b_x = b[0].as_f64().unwrap() as f32;
            let b_y = b[1].as_f64().unwrap() as f32;
            let b_z = b[2].as_f64().unwrap() as f32;

            current
                [object["location"][2].as_i64().unwrap() as usize]
                [object["location"][1].as_i64().unwrap() as usize]
                [object["location"][0].as_i64().unwrap() as usize]
            = SpaceData {
                e: Field3Vec{ components:vec![
                    e_x, e_y, e_z
                ] },
                b: Field3Vec{ components:vec![
                    b_x, b_y, b_z
                ] },
                ..SpaceData::default()
            };
        } else if object_type == "plane" {
            let axis = object["axis"].as_str().unwrap();
            let location = object["location"].as_i64().unwrap() as usize;
            let e = object["E"].as_array().unwrap();
            let e_x = e[0].as_f64().unwrap() as f32;
            let e_y = e[1].as_f64().unwrap() as f32;
            let e_z = e[2].as_f64().unwrap() as f32;

            let b = object["B"].as_array().unwrap();
            let b_x = b[0].as_f64().unwrap() as f32;
            let b_y = b[1].as_f64().unwrap() as f32;
            let b_z = b[2].as_f64().unwrap() as f32;

            for i in 0..(LATICE_DENSITY * SIMULATION_SIDE_LENGTH) {
                for j in 0..(LATICE_DENSITY * SIMULATION_SIDE_LENGTH) {
                    if axis == "x" {
                        current[i as usize][j as usize][location] = SpaceData {
                            e: Field3Vec{ components:vec![e_x, e_y, e_z] },
                            b: Field3Vec{ components:vec![b_x, b_y, b_z] },
                            ..SpaceData::default()
                        };
                    } else if axis == "y" {
                        current[i as usize][location][j as usize] = SpaceData {
                            e: Field3Vec{ components:vec![e_x, e_y, e_z] },
                            b: Field3Vec{ components:vec![b_x, b_y, b_z] },
                            ..SpaceData::default()
                        };
                    } else if axis == "z" {
                        current[location][j as usize][i as usize] = SpaceData {
                            e: Field3Vec{ components:vec![e_x, e_y, e_z] },
                            b: Field3Vec{ components:vec![b_x, b_y, b_z] },
                            ..SpaceData::default()
                        };
                    }
                    
                }
            }
        } else if object_type == "wire" {
            let axis = object["axis"].as_str().unwrap();
            let location = object["location"].as_array().unwrap();
            for i in 0..(LATICE_DENSITY * SIMULATION_SIDE_LENGTH) {
                if axis == "x" {
                    current[location[1].as_i64().unwrap() as usize][location[0].as_i64().unwrap() as usize][i as usize] = SpaceData {
                        object_index: current_field_object_index,
                        ..current[location[1].as_i64().unwrap() as usize][location[0].as_i64().unwrap() as usize][i as usize].clone()
                    };
                    field_object = Box::new(Wire{ 
                        amplitude: object["amplitude"].as_f64().unwrap() as f32,
                        angular_frequency: object["angular_frequency"].as_f64().unwrap() as f32,
                        direction: Field3Vec{components: vec![1.0, 0.0, 0.0]}
                    });
                    field_objects.push(field_object);
                } else if axis == "y" {
                    current[location[1].as_i64().unwrap() as usize][i as usize][location[0].as_i64().unwrap() as usize] = SpaceData {
                        object_index: current_field_object_index,
                        ..current[location[1].as_i64().unwrap() as usize][i as usize][location[0].as_i64().unwrap() as usize].clone()
                    };
                    field_object = Box::new(Wire{ 
                        amplitude: object["amplitude"].as_f64().unwrap() as f32,
                        angular_frequency: object["angular_frequency"].as_f64().unwrap() as f32,
                        direction: Field3Vec{components: vec![0.0, 1.0, 0.0]}
                    });
                    field_objects.push(field_object);
                } else if axis == "z" {
                    current[i as usize][location[1].as_i64().unwrap() as usize][location[0].as_i64().unwrap() as usize] = SpaceData {
                        object_index: current_field_object_index,
                        ..current[i as usize][location[1].as_i64().unwrap() as usize][location[0].as_i64().unwrap() as usize].clone()
                    };
                    field_object = Box::new(Wire{ 
                        amplitude: object["amplitude"].as_f64().unwrap() as f32,
                        angular_frequency: object["angular_frequency"].as_f64().unwrap() as f32,
                        direction: Field3Vec{components: vec![0.0, 0.0, 1.0]}
                    });
                    field_objects.push(field_object);
                }
            }
            current_field_object_index += 1;
        }
    }
    /*for i in 2..(LATICE_DENSITY * SIMULATION_SIDE_LENGTH -2) {
        for j in 2..(LATICE_DENSITY * SIMULATION_SIDE_LENGTH -2) {
            current[i as usize][j as usize][i as usize] = SpaceData {
                e: Field3Vec{ components:vec![1.0, 0.0, 0.0] },
                b: Field3Vec{ components:vec![0.0, 0.0, 0.0] },
            };
        }
    }*/
    /*current[15][15][15] = SpaceData {
        e: Field3Vec{ components:vec![1.0, 0.0, 0.0] },
        b: Field3Vec{ components:vec![0.0, 0.0, 0.0] },
    };
    current[17][15][15] = SpaceData {
        e: Field3Vec{ components:vec![-1.0, 0.0, 0.0] },
        b: Field3Vec{ components:vec![0.0, 0.0, 0.0] },
    };
    current[16][15][16] = SpaceData {
        e: Field3Vec{ components:vec![0.0, 0.0, 1.0] },
        b: Field3Vec{ components:vec![0.0, 0.0, 0.0] },
    };
    current[16][15][14] = SpaceData {
        e: Field3Vec{ components:vec![0.0, 0.0, -1.0] },
        b: Field3Vec{ components:vec![0.0, 0.0, 0.0] },
    };
    current[15][15][15] = SpaceData {
        e: Field3Vec{ components:vec![1.0, 0.0, 0.0] },
        b: Field3Vec{ components:vec![0.0, 0.0, 0.0] },
    };*/

    //let mut totalDivergence:f32 = 0.0;

    // Send initial conditions through pipeline
    for (z, plane) in current.iter().enumerate() {
        for (y, row) in plane.iter().enumerate() {
            for (x, node) in row.iter().enumerate() {
                let _ = out_writer.write_all(&node.e.components[0].to_le_bytes());
                let _ = out_writer.write_all(&[0]);
                let _ = out_writer.write_all(&node.e.components[1].to_le_bytes());
                let _ = out_writer.write_all(&[0]);
                let _ = out_writer.write_all(&node.e.components[2].to_le_bytes());
                let _ = out_writer.write_all(&[1]);
                let _ = out_writer.write_all(&node.b.components[0].to_le_bytes());
                let _ = out_writer.write_all(&[0]);
                let _ = out_writer.write_all(&node.b.components[1].to_le_bytes());
                let _ = out_writer.write_all(&[0]);
                let _ = out_writer.write_all(&node.b.components[2].to_le_bytes());
                if z == current.len()-1 && y == plane.len()-1 && x == row.len()-1 {
                    let _ = out_writer.write_all(&[5]);
                } else if y == plane.len()-1 && x == row.len()-1 {
                    let _ = out_writer.write_all(&[4]);
                } else if x == row.len()-1 {
                    let _ = out_writer.write_all(&[3]);
                } else {
                    let _ = out_writer.write_all(&[2]);
                }
            }
        }
    }

    // Begin simulation (1 step is used for the initial conditions)
    let mut steps_left = steps-1;
    let mut field_object_currents: Vec<Field3Vec> = vec![];
    for field_object in &field_objects {
        field_object_currents.push(field_object.currrent_density(0.0));
    }
        
    while steps_left > 0 {

        for (i, field_object) in field_objects.iter().enumerate() {
            field_object_currents[i] = field_object.currrent_density((steps - steps_left) as f32 * dt);
        }

        for (z, plane) in current.iter().enumerate() {
            for (y, row) in plane.iter().enumerate() {
                for (x, node) in row.iter().enumerate() {

                    let derivative_e_x: Field3Vec;
                    let derivative_b_x: Field3Vec;
                    if x == 0 {
                        match boundary_condition {
                            BoundaryCondition::Fit => {
                                derivative_e_x = (&current[z][y][x+1].e - &node.e) * LATICE_DENSITY as f32;
                                derivative_b_x = (&current[z][y][x+1].b - &node.b) * LATICE_DENSITY as f32;
                            }
                            BoundaryCondition::Clip => {
                                derivative_e_x = (&current[z][y][x+1].e) * LATICE_DENSITY as f32 * 0.5;
                                derivative_b_x = (&current[z][y][x+1].b) * LATICE_DENSITY as f32 * 0.5;
                            }
                        }
                    } else if x == row.len() - 1 {
                        match boundary_condition {
                            BoundaryCondition::Fit => {
                                derivative_e_x = (&node.e - &current[z][y][x-1].e) * LATICE_DENSITY as f32;
                                derivative_b_x = (&node.b - &current[z][y][x-1].b) * LATICE_DENSITY as f32;
                            }
                            BoundaryCondition::Clip => {
                                derivative_e_x = &current[z][y][x-1].e * LATICE_DENSITY as f32 * -0.5;
                                derivative_b_x = &current[z][y][x-1].b * LATICE_DENSITY as f32 * -0.5;
                            }
                        }
                    } else {
                        derivative_e_x = (&current[z][y][x+1].e - &current[z][y][x-1].e) * LATICE_DENSITY as f32 * 0.5;
                        derivative_b_x = (&current[z][y][x+1].b - &current[z][y][x-1].b) * LATICE_DENSITY as f32 * 0.5;
                    }

                    let derivative_e_y: Field3Vec;
                    let derivative_b_y: Field3Vec;
                    if y == 0 {
                        match boundary_condition {
                            BoundaryCondition::Fit => {
                                derivative_e_y = (&current[z][y+1][x].e - &node.e) * LATICE_DENSITY as f32;
                                derivative_b_y = (&current[z][y+1][x].b - &node.b) * LATICE_DENSITY as f32;
                            }
                            BoundaryCondition::Clip => {
                                derivative_e_y = (&current[z][y+1][x].e) * LATICE_DENSITY as f32 * 0.5;
                                derivative_b_y = (&current[z][y+1][x].b) * LATICE_DENSITY as f32 * 0.5;
                            }
                        }
                    } else if y == row.len() - 1 {
                        match boundary_condition {
                            BoundaryCondition::Fit => {
                                derivative_e_y = (&node.e - &current[z][y-1][x].e) * LATICE_DENSITY as f32;
                                derivative_b_y = (&node.b - &current[z][y-1][x].b) * LATICE_DENSITY as f32;
                            }
                            BoundaryCondition::Clip => {
                                derivative_e_y = &current[z][y-1][x].e * LATICE_DENSITY as f32 * -0.5;
                                derivative_b_y = &current[z][y-1][x].b * LATICE_DENSITY as f32 * -0.5;
                            }
                        }
                    } else {
                        derivative_e_y = (&current[z][y+1][x].e - &current[z][y-1][x].e) * LATICE_DENSITY as f32 * 0.5;
                        derivative_b_y = (&current[z][y+1][x].b - &current[z][y-1][x].b) * LATICE_DENSITY as f32 * 0.5;
                    }

                    let derivative_e_z: Field3Vec;
                    let derivative_b_z: Field3Vec;
                    if z == 0 {
                        match boundary_condition {
                            BoundaryCondition::Fit => {
                                derivative_e_z = (&current[z+1][y][x].e - &node.e) * LATICE_DENSITY as f32;
                                derivative_b_z = (&current[z+1][y][x].b - &node.b) * LATICE_DENSITY as f32;
                            }
                            BoundaryCondition::Clip => {
                                derivative_e_z = (&current[z+1][y][x].e) * LATICE_DENSITY as f32 * 0.5;
                                derivative_b_z = (&current[z+1][y][x].b) * LATICE_DENSITY as f32 * 0.5;
                            }
                        } 
                    } else if z == row.len() - 1 {
                        match boundary_condition {
                            BoundaryCondition::Fit => {
                                derivative_e_z = (&node.e - &current[z-1][y][x].e) * LATICE_DENSITY as f32;
                                derivative_b_z = (&node.b - &current[z-1][y][x].b) * LATICE_DENSITY as f32;
                            }
                            BoundaryCondition::Clip => {
                                derivative_e_z = &current[z-1][y][x].e * LATICE_DENSITY as f32 * -0.5;
                                derivative_b_z = &current[z-1][y][x].b * LATICE_DENSITY as f32 * -0.5;
                            }
                        }
                    } else {
                        derivative_e_z = (&current[z+1][y][x].e - &current[z-1][y][x].e) * LATICE_DENSITY as f32 * 0.5;
                        derivative_b_z = (&current[z+1][y][x].b - &current[z-1][y][x].b) * LATICE_DENSITY as f32 * 0.5;
                    }

                    let curl_e = Field3Vec { components: vec![
                        derivative_e_y.components[2] - derivative_e_z.components[1],
                        derivative_e_z.components[0] - derivative_e_x.components[2],
                        derivative_e_x.components[1] - derivative_e_y.components[0]
                    ] };

                    let curl_b = Field3Vec { components: vec![
                        derivative_b_y.components[2] - derivative_b_z.components[1],
                        derivative_b_z.components[0] - derivative_b_x.components[2],
                        derivative_b_x.components[1] - derivative_b_y.components[0]
                    ] };


                    //totalDivergence += (derivative_e_x.components[0] + derivative_e_y.components[1] + derivative_e_z.components[2]).abs();
                    //totalDivergence += (derivative_b_x.components[0] + derivative_b_y.components[1] + derivative_b_z.components[2]).abs();


                    let new_node = SpaceData {
                        b: &node.b - &(dt * curl_e),
                        e: &node.e + &(dt * ((curl_b / (e0 * m0)) - (&field_object_currents[node.object_index] / e0))),
                        object_index: node.object_index,
                    };

                    /*if node.object_index > 0 {
                        eprintln!("i: {}", node.object_index);
                        eprintln!("step: {}", steps - steps_left);
                        eprintln!("J: {}", &field_object_currents[node.object_index] / e0);
                    }*/
                    

                    next[z][y][x] = new_node.clone();


                    if 
                        (steps-steps_left) % time_culling_factor == 0 && 
                        (x+1) % space_culling_factor as usize == 0 &&
                        (y+1) % space_culling_factor as usize == 0 &&
                        (z+1) % space_culling_factor as usize == 0
                    {
                        let _ = out_writer.write_all(&new_node.e.components[0].to_le_bytes());
                        let _ = out_writer.write_all(&[0]);
                        let _ = out_writer.write_all(&new_node.e.components[1].to_le_bytes());
                        let _ = out_writer.write_all(&[0]);
                        let _ = out_writer.write_all(&new_node.e.components[2].to_le_bytes());
                        let _ = out_writer.write_all(&[1]);
                        let _ = out_writer.write_all(&new_node.b.components[0].to_le_bytes());
                        let _ = out_writer.write_all(&[0]);
                        let _ = out_writer.write_all(&new_node.b.components[1].to_le_bytes());
                        let _ = out_writer.write_all(&[0]);
                        let _ = out_writer.write_all(&new_node.b.components[2].to_le_bytes());
                        if z == current.len()-1 && y == plane.len()-1 && x == row.len()-1 {
                            let _ = out_writer.write_all(&[5]);
                        } else if y == plane.len()-1 && x == row.len()-1 {
                            let _ = out_writer.write_all(&[4]);
                        } else if x == row.len()-1 {
                            let _ = out_writer.write_all(&[3]);
                        } else {
                            let _ = out_writer.write_all(&[2]);
                        }
                    }
                }
            }
        }
        //eprintln!("Divergence: {}", totalDivergence);

        //totalDivergence = 0.0;

        (current, next) = (next, current);
        steps_left -= 1;
        update_progress_bar(steps - steps_left, steps, "Running simulation... ");
    }
    eprintln!();

    eprintln!("Simulation Done");

    ExitCode::SUCCESS
}
