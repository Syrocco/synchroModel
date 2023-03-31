use std::fs::File;
use std::io::{Write, Error};

#[derive(Copy, Clone)]
struct Particle{
    z: f64,
    v: f64,
    r: f64,
}

#[derive(Copy, Clone)]
struct Plate{
    w: f64,
    amp: f64,
    h: f64,
    res: f64,
    g: f64,
}

struct System{
    plate: Plate,
    particles: Vec<Particle>,
    dt: f64,
    step: i32,
    t: f64
}


impl Particle{
    fn new(z: f64, v: f64, r: f64) -> Self{
        Particle{z, v, r}
    }
}

impl Plate{
    fn new(w: f64, amp: f64, h: f64, res: f64, g: f64) -> Self{
        Plate{w, amp, h, res, g}
    }

    fn get_floor(&self, t: f64) -> f64{
        self.amp*(self.w*t).sin()
    }

    fn get_ceil(&self, t: f64) -> f64{
        self.amp*(self.w*t).sin() + self.h
    }

    fn get_velocity(&self, t: f64) -> f64{
        self.amp*self.w*(self.w*t).cos()
    }
}

impl System{
    fn new(plate: Plate, particles: Vec<Particle>, dt: f64) -> Self{
        System{plate, particles, dt, step: 0, t: 0.}
    }

    fn get_collision_floor(&self) -> Vec<f64>{
        self.particles.iter().map(|x| self.plate.get_floor(self.t) + x.r).collect()
    }

    fn get_collision_ceil(&self) -> Vec<f64>{
        self.particles.iter().map(|x| self.plate.get_ceil(self.t) - x.r).collect()
    }

    fn integrate(&mut self){
        for particle in self.particles.iter_mut(){
            particle.z += particle.v*self.dt + self.plate.g*self.dt*self.dt/2.;
            particle.v +=  self.plate.g*self.dt;
        }
    }

    fn back_in_box(&mut self){
        let ceils = self.get_collision_ceil();
        let floors = self.get_collision_floor();
        for ((particle, &ceil), &floor) in self.particles.iter_mut().zip(ceils.iter()).zip(floors.iter()){
            if particle.z > ceil {
                particle.z += -2.*(particle.z - ceil);
                particle.v += -(self.plate.res + 1.)*(particle.v - self.plate.get_velocity(self.t));
            }
            else if particle.z < floor{
                particle.z += -2.*(particle.z - floor);
                particle.v += -(self.plate.res + 1.)*(particle.v - self.plate.get_velocity(self.t));
            }    
        }
    }

    fn update(&mut self){
        self.step += 1;
        self.t += self.dt;
        self.integrate();
        self.back_in_box();
    }
}

impl Iterator for System{
    type Item = (f64, Vec<f64>, Vec<f64>);
    fn next(&mut self) -> Option<Self::Item>{
        self.update();
        Some((self.t, self.particles.iter().map(|p| p.z).collect(), self.particles.iter().map(|p| p.v).collect()))
    }
}





fn main() -> Result<(), Error>{
    let h = 2.;
    let amp = 0.5;
    let w = 3.;
    let res = 0.5;
    let g = -0.5;

    let r = 0.5;
    let z = r + 0.1;

    let dt = 0.01;

    let particle1 = Particle::new(z, 0., r);
    let particle2 = Particle::new(z, 1., r);
    let plate = Plate::new(w, amp, h, res, g);
    let mut system = System::new(plate, vec![particle1, particle2], dt);
    

    for _a in system.by_ref().take(100000){
        continue
    }
    
    let path = "data.txt";
    let mut output = File::create(path)?;
    for (t, p, v) in system.by_ref().take(10000){
        write!(output,"{} {} {} {} {} {} {}\n", t, p[0], v[0], p[1], v[1], plate.get_ceil(t) + h - r, plate.get_floor(t) + r)?;
    } 
    Ok(())
}
