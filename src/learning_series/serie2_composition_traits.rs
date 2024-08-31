use std::cell::RefCell;
use std::ops::{Add, Mul};
use std::fmt::Display;

use chrono::{DateTime, Utc};

pub trait Great {
    fn great(&self) {
        println!("Hello there!");
    }
}

#[derive(Clone)]
pub struct Pet {
    pub name: String,
    pub specie: String,
    pub breed: String,
    pub age_months: i32,
}

pub struct WildAnimal {
    pub specie: String,
    pub habitad: String,
}

pub struct Plant {
}

#[derive(Clone, Copy)]
pub struct Complex {
    pub a: f32,
    pub b: f32, 
}

pub struct Diagnostic {
    pub created_date: DateTime<Utc>,
    pub description: String,
}

pub struct Prescription {
    pub created_date: DateTime<Utc>,
    pub name: String,
    pub dosis: String,
}

pub struct PetRecord {
    pub pet_info: Pet,
    pub diagnostics: Vec<Diagnostic>,
    pub prescriptions: Vec<Prescription>,
}

pub trait Describe : Display {
    fn describe(&self);
}

pub struct FrameBox{ pub x: i32, pub y: i32, pub width: i32, pub height: i32 }

pub trait Shape {
    fn perimeter(&self) -> i32;
    fn area(&self) -> i32;
    fn framebox(&self) -> FrameBox; 
}

pub trait Drawable : Shape {
    fn draw(&self);
}

pub struct Square {
    pub x: i32,
    pub y: i32,
    pub width: i32,
    pub height: i32,
}

impl Great for Pet {
    fn great(&self) {
        println!("Hello {}, you are awesome!", self.name);
    }
}

impl Great for WildAnimal {
    fn great(&self)
    {
        println!("Hello {}, you are amazing wild!", self.specie)
    }
}

impl Great for Plant {}

// If shape hasn't been implemented Drawable implementation would fail.
impl Shape for Square {
    fn perimeter(&self) -> i32 {
        self.height * 2 + self.width * 2
    }

    fn area(&self) -> i32 {
        self.height * self.width
    }

    fn framebox(&self) -> FrameBox {
        FrameBox {
            x: self.x,
            y: self.y,
            width: self.width,
            height: self.height,
        }
    }
}

impl Drawable for Square {
    fn draw(&self) {
        println!(
            "Drawing square | x: {}, y: {}, width: {}, height: {}, perimeter: {}, area: {}",
            self.framebox().x, self.framebox().y, self.framebox().width, 
            self.framebox().height, self.perimeter(), self.area()
        );
    }
}

impl Display for Pet {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f,
            "Pet | name: {}, specie: {}, breed: {}, age {} months",
            self.name, self.specie, self.breed, self.age_months
        )?;

        Ok(())
    }
}

impl Describe for Pet {
    fn describe(&self) {
        println!("{}", self); // We can print Pet directly because implements Pet.
    }
}

pub fn describe<T:Describe>(obj: T) {
    obj.describe();
}

pub fn great<T:Great>(obj: &T) {
    obj.great();
}

pub fn great2(obj: impl Great) -> impl Great {
    obj
}

impl<'a> Mul<&'a Complex> for &'a Complex {
    type Output = Complex;

    fn mul(self, rhs: &'a Complex) -> Self::Output {
        Self::Output {
            a: self.a * rhs.a + self.b * rhs.b * -1.,
            b: self.a * rhs.b + self.b * rhs.a,
        }
    }
}

impl Mul for Complex {
    type Output = Complex;

    fn mul(self, rhs: Complex) -> Self::Output {
        Self::Output {
            a: self.a * rhs.a + self.b * rhs.b * -1.,
            b: self.a * rhs.b + self.b * rhs.a,
        }
    }
}

impl Add for Complex {
    type Output = Complex;

    fn add(self, rhs: Self) -> Self::Output {
        Self::Output {
            a: self.a + rhs.a,
            b: self.b + rhs.b,
        }
    }
}

impl PetRecord {
    pub fn new(name: &str, specie: &str, breed: &str, age: i32 ) -> Self {
        Self {
            pet_info: Pet {
                name: String::from(name),
                specie: String::from(specie),
                breed: String::from(breed),
                age_months: age,
            },
            diagnostics: Vec::new(),
            prescriptions: Vec::new(),
        }
    }

    pub fn add_prescription(&mut self, prescription: Prescription) {
        self.prescriptions.push(prescription);
    }

    pub fn add_diagnostic(&mut self, diagnostic: Diagnostic) {
        self.diagnostics.push(diagnostic);
    }

    pub fn summarize(&self) {
        println!("{} {} months", self.pet_info.name, self.pet_info.age_months);
        println!("____________________________________");
        println!("Especie: {}, {}", self.pet_info.specie, self.pet_info.breed);

        println!();

        println!("Diagnogsis");

        println!("____________________________________");

        for diagnogtic in &self.diagnostics {
            println!("{}", diagnogtic.created_date);
            println!("--------------");
            println!("{}", diagnogtic.description);
        }

        println!();

        println!("Prescriptions");

        println!("____________________________________");

        for prescription in &self.prescriptions {
            println!("{}", prescription.created_date);
            println!("--------------");
            println!("name: {}, dosis: {}", prescription.name, prescription.dosis);
        }
    }
}

impl Display for PetRecord {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Ok(())
    }
}
impl Describe for PetRecord {
    fn describe(&self) {
        self.summarize();
    }
}

#[cfg(test)]
mod tests {
    use crate::learning_series::serie2_composition_traits::*;

    #[test]
    fn exercise_01() {
        let pet = Pet {
            name: String::from("mezcal"),
            specie: String::from("Dog"),
            breed: String::from("Golden Retriever"),
            age_months: 24,
        };

        pet.describe();
    }

    #[test]
    fn exercise_02() {
        let pet = Pet {
            name: String::from("mezcal"),
            specie: String::from("Dog"),
            breed: String::from("Golden Retriever"),
            age_months: 24,
        };

        let _wild_animal = WildAnimal {
            specie: String::from("lyon"),
            habitad: String::from("woodlands"),
        };

        describe(pet);
        
        // Uncommenting the follow wild fail because
        // This fails because wild animal doesn't satify
        // Describe trait bound.
        // describe(_wild_animal);  
    }

    #[test]
    fn exercise_03() {
        let square = Square {
            x: 10,
            y: 10,
            width: 20,
            height: 20,
        };

        square.draw();
    }

    #[test]
    fn exercise_04() {
        let pet = Pet {
            name: String::from("mezcal"),
            specie: String::from("Dog"),
            breed: String::from("Golden Retriever"),
            age_months: 24,
        };
        
        let wild_animal = WildAnimal {
            specie: String::from("lyon"),
            habitad: String::from("woodlands"),
        };

        let plant = Plant {};

        great(&pet);

        great2(pet.clone()).great();

        pet.great();

        wild_animal.great();

        plant.great();
    }

    #[test]
    fn exercise_05() {
        let pet = Pet {
            name: String::from("mezcal"),
            specie: String::from("Dog"),
            breed: String::from("Golden Retriever"),
            age_months: 24,
        };
        
        let wild_animal = WildAnimal {
            specie: String::from("lyon"),
            habitad: String::from("woodlands"),
        };

        let plant = Plant {};

        let _square = Square {
            x: 10,
            y: 10,
            width: 20,
            height: 20,
        };
        
        // Uncommenting Box::new(_square) will fail because it doesn't implement Great
        let greatings:Vec<Box<dyn Great>> = vec![
            Box::new(pet),
            Box::new(wild_animal),
            Box::new(plant),
            // Box::new(_square),
        ];

        for greatable in greatings {
            greatable.great();
        }
    }

    #[test]
    fn exercise_06() {
        let z1 = Complex {
            a: 1.0,
            b: 2.0 
        };

        let z2 = Complex {
            a: 2.0,
            b: 4.0,
        };

        let zr = z1 * z2;
        let zr2 = z1 + z2;
        let _zr3 = &z1 * &z2;

        assert_eq!(zr.a, -6.);
        assert_eq!(zr.b, 8.);

        assert_eq!(zr2.a, 3.);
        assert_eq!(zr2.a, 3.);
    }

    #[test]
    fn exercise_07() {
        let mut pet_record = PetRecord::new(
            "mezcal",
            "dog",
            "golden",
            36
        );

        pet_record.add_diagnostic(Diagnostic {
            created_date: Utc::now(),
            description: String::from("stomac ache!"),
        });

        pet_record.add_prescription(Prescription {
            created_date: Utc::now(),
            name: String::from("Sucralfate"),
            dosis: String::from("10mg"),
        });

        pet_record.summarize();
    }

    #[test]
    fn exercise_08() {
        let mut pet_record = PetRecord::new(
            "mezcal",
            "dog",
            "golden",
            36
        );

        pet_record.add_diagnostic(Diagnostic {
            created_date: Utc::now(),
            description: String::from("stomac ache!"),
        });

        pet_record.add_prescription(Prescription {
            created_date: Utc::now(),
            name: String::from("Sucralfate"),
            dosis: String::from("10mg"),
        });

        pet_record.describe();
    }
}