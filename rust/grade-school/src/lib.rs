pub struct School(Vec<(u32, Vec<String>)>);

impl School {
    pub fn new() -> School {
        School(Vec::new())
    }

    pub fn add(&mut self, grade: u32, student: &str) {
        for (grd, students_vec) in self.0.iter_mut() {
            if *grd == grade {
                students_vec.push(student.to_string());
                students_vec.sort_unstable();
                return;
            }
        }

        // add a new grade to our school db
        self.0.push((grade, vec![student.to_string()]));
        self.0.sort_unstable_by(|a, b| a.0.partial_cmp(&b.0).unwrap())
    }

    pub fn grades(&self) -> Vec<u32> {
        self.0.iter().map(|v| v.0).collect::<Vec<u32>>()
    }

    pub fn grade(&self, grade: u32) -> Vec<String> {
        for (grd, students_vec) in self.0.iter() {
            if *grd == grade {
                return students_vec.clone();
            }
        }
        vec![]
    }
}
