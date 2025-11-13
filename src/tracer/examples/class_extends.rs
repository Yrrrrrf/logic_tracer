// examples/class_extends.rs

// Macro ultra-simplificada para simular herencia
macro_rules! class_extends {
    // Versión mínima que solo maneja campos
    ($child:ident extends $parent:ident { $($field:ident : $type:ty),* }) => {
        // Estructura hija
        pub struct $child {
            parent: $parent,
            $(pub $field: $type),*
        }

        // Constructor básico
        impl $child {
            pub fn new(parent: $parent, $($field: $type),*) -> Self {
                Self {
                    parent,
                    $($field),*
                }
            }
        }

        // Delegación mediante Deref
        impl std::ops::Deref for $child {
            type Target = $parent;

            fn deref(&self) -> &Self::Target {
                &self.parent
            }
        }
    };
}

// Definición de clase base
pub struct Person {
    pub name: String,
    pub age: u32,
}

impl Person {
    pub fn new(name: String, age: u32) -> Self {
        Self { name, age }
    }

    pub fn greet(&self) {
        println!("Hello, my name is {}", self.name);
    }
}

// Uso de la macro
class_extends!(Student extends Person {
    school: String,
    grade: u8
});

// Implementación manual de métodos para Student
impl Student {
    pub fn study(&self) {
        println!("{} is studying at {}", self.parent.name, self.school);
    }
}

fn main() {
    let person = Person::new("John".to_string(), 30);
    person.greet();

    let student = Student::new(Person::new("Alice".to_string(), 20), "MIT".to_string(), 10);

    // Método "heredado" mediante Deref
    student.greet();

    // Método propio
    student.study();
    student.parent.greet();
}
