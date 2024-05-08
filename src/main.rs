use std::f64::consts::PI;
use std::fmt::write;
use std::fs::File;
use std::io::{Write, Error};
use num_complex::{self, Complex64};

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
    k: f64,
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
    fn new(w: f64, amp: f64, h: f64, res: f64, g: f64, k: f64) -> Self{
        Plate{w, amp, h, res, g, k}
    }

    fn get_floor(&self, t: f64) -> f64{
        self.amp*(self.w*t).sin()
    }

    fn get_ceil(&self, t: f64) -> f64{
        self.amp*(self.w*t).sin() + self.h
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

        let ceils = self.get_collision_ceil();
        let floors = self.get_collision_floor();
        
        for ((particle, &ceil), &floor) in self.particles.iter_mut().zip(ceils.iter()).zip(floors.iter()){
            let mut f = self.plate.g;
            if particle.z > ceil {
                f +=  -(particle.z - ceil).abs()*self.plate.k - particle.v*self.plate.res;
            }
            else if particle.z < floor {
                f +=  (particle.z - floor).abs()*self.plate.k - particle.v*self.plate.res;
            }
            
            particle.v +=  f*self.dt;
            particle.z += particle.v*self.dt;
        }
    }


    fn update(&mut self){
        self.step += 1;
        self.t += self.dt;
        self.integrate();
        //self.back_in_box();
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


    let h = 22./10.;
    let amp = 11./11.;
    let w = 1.;
    let res = 1.;
    let k = 1001.;
    let g = -0.1;

    let r = 0.5;
    let z = r + 0.1;

    let dt = 0.01;

    let particle1 = Particle::new(z, -10., r);
    let particle2 = Particle::new(z, 7., r);
    let particle3 = Particle::new(z, -1., r);
    let particle4 = Particle::new(z, 15., r);
    let plate = Plate::new(w, amp, h, res, g, k);
    let mut system = System::new(plate, vec![particle1, particle2, particle3, particle4], dt);
    

    for _a in system.by_ref().take(50000000){
        continue
    }
    
    let path = "data.txt";
    let mut output = File::create(path)?;
    for (t, p, v) in system.by_ref().take(10000){
        writeln!(output,"{} {} {} {} {} {} {}", t, p[0], p[1], p[2], p[3], plate.get_ceil(t) - r, plate.get_floor(t) + r)?;
    } 
    println!("done");
    Ok(())
}


/* 
fn main() -> Result<(), Error>{
    let path = "data.txt";
    let mut output = File::create(path)?;
    for h in (20..40).step_by(2).map(|x| x as f64/16.){
        println!("{h}");
        for amp in (1..20).step_by(2).map(|x| x as f64/11.){
            let w = 1.;
            let res = 0.1;
            let k = 1001.;
            let g = -0.1;

            let r = 0.5;
            let z = r + 0.1;

            let dt = 0.01;
            let particle1 = Particle::new(z, -3., r);
            let particle2 = Particle::new(z, 3., r);
            let particle3 = Particle::new(z, -4., r);
            let particle4 = Particle::new(z, 1., r);
            let plate = Plate::new(w, amp, h, res, g, k);
            let mut system = System::new(plate, vec![particle1, particle2, particle3, particle4], dt);
            

            for _a in system.by_ref().take(10000000){
                continue
            }
            
            let n = 10000;
            let mut diff = 0.;
            let mut s = 0;
            for (t, p, v) in system.by_ref().take(n){
                s += 1;
                diff += p.iter().map(|z| (z - (-amp + r))/(amp - r)*PI).map(|z| Complex64::new(0., z).exp()).sum::<Complex64>().norm()/4.;
            } 

            write!(output,"{}\t", diff/(n as f64))?;
        }
        writeln!(output, "")?;
    }
    println!("done");
    Ok(())
}*/
