use std::{collections::HashMap, error::Error, fs, io::stdin};

pub fn run() -> Result<(), Box<dyn Error>> {
    println!("WELCOME TO STUDENT MANAGEMENT");

    let mut student_management = StudentManagement::new();

    loop {
        println!("1.Create");
        println!("2.Find and Sort");
        println!("3.Update/Delete");
        println!("4.Report");
        println!("5.Load from file");
        println!("6.Save");
        println!("7.Exit");
        println!("(Please choose 1 to Create, 2 to Find and Sort, 3 to Update/Delete, 4 to Report, 5 to Load from file, 6 to Save and 7 to Exit program).");

        let mut buffer = String::new();
        match stdin().read_line(&mut buffer) {
            Ok(_) => {}
            Err(e) => {
                eprintln!("{}", e);
                continue;
            }
        };

        let option: u8 = match buffer.trim().parse() {
            Ok(val) => val,
            Err(e) => {
                eprintln!("Unable to read iput: {}", e);
                continue;
            }
        };

        match option {
            1 => {
                let mut count = 0;

                'outer: loop {
                    if count > 1 {
                        let mut inner_option = String::new();

                        loop {
                            println!("Do you want to continue (Y/N)? Choose Y to continue, N to return main screen.");

                            match stdin().read_line(&mut inner_option) {
                                Ok(_) => {}
                                Err(_) => {
                                    eprintln!("Invalid input.");
                                    continue;
                                }
                            }

                            match inner_option.trim() {
                                "Y" => {
                                    break;
                                }
                                "N" => {
                                    break 'outer;
                                }
                                _ => {
                                    eprintln!("Please input only Y or N.");
                                    inner_option.clear();
                                }
                            }
                        }
                    }

                    student_management.add(
                        parse_student_name(),
                        parse_semester(),
                        parse_course_name(),
                    );
                    count += 1;

                    println!();
                }
            }
            2 => {
                let mut name = String::new();
                loop {
                    println!("Enter student name:");
                    match stdin().read_line(&mut name) {
                        Ok(_) => {
                            break;
                        }
                        Err(_) => {
                            eprintln!("Invalid input.");
                            continue;
                        }
                    }
                }

                for (student_name, semester, course_name) in student_management.search(name.trim())
                {
                    println!("{student_name}|{semester}|{course_name}")
                }
                println!();
            }
            3 => {
                let student_id: i32;

                loop {
                    let mut student_id_buffer = String::new();

                    println!("Enter student id:");
                    match stdin().read_line(&mut student_id_buffer) {
                        Ok(_) => {
                            student_id = match student_id_buffer.trim().parse() {
                                Ok(val) => val,
                                Err(_) => {
                                    eprintln!("Invalid input.");
                                    continue;
                                }
                            };

                            break;
                        }
                        Err(e) => {
                            eprintln!("Unable to read iput: {}", e);
                            continue;
                        }
                    }
                }

                let index = match student_management.search_by_id(student_id) {
                    Ok(val) => val,
                    Err(e) => {
                        eprintln!("Error searching for student with id {student_id}: {e}");
                        break;
                    }
                };

                let mut inner_option = String::new();

                loop {
                    println!("Do you want to update (U) or delete (D) student.");

                    match stdin().read_line(&mut inner_option) {
                        Ok(_) => match inner_option.trim() {
                            "U" => {
                                student_management.update(
                                    index,
                                    parse_student_name(),
                                    parse_semester(),
                                    parse_course_name(),
                                );
                                break;
                            }
                            "D" => {
                                student_management.delete(index);
                                break;
                            }
                            _ => {
                                eprintln!("Please input only U or D.");
                                inner_option.clear();
                            }
                        },
                        Err(e) => {
                            eprintln!("Unable to read iput: {}", e);
                            continue;
                        }
                    }
                }
            }
            4 => {
                for (student_name, course_name, course_count) in student_management.report() {
                    println!("{student_name}|{course_name}|{course_count}")
                }
                println!();
            }
            5 => {
                let mut file_path = String::new();

                println!("Enter file path: ");
                match stdin().read_line(&mut file_path) {
                    Ok(_) => {}
                    Err(e) => {
                        eprintln!("Unable to read iput: {}", e);
                        continue;
                    }
                }

                let content = match fs::read_to_string(file_path.trim()) {
                    Ok(value) => value,
                    Err(e) => {
                        eprintln!("Cannot read file with filepath {}: {}", &file_path, e);
                        continue;
                    }
                };

                student_management.load(content);
            }
            6 => {
                let mut file_path = String::new();

                println!("Enter file path: ");
                match stdin().read_line(&mut file_path) {
                    Ok(_) => {}
                    Err(e) => {
                        eprintln!("Unable to read iput: {}", e);
                        continue;
                    }
                }

                match student_management.save(file_path.trim()) {
                    Ok(_) => {
                        println!("Save successfully")
                    }
                    Err(e) => {
                        eprintln!("Save failed: {}", e)
                    }
                };
            }
            7 => {
                break;
            }
            _ => {}
        };
    }

    Ok(())
}

fn parse_semester() -> String {
    let mut semester = String::new();
    loop {
        println!("Enter semester:");
        match stdin().read_line(&mut semester) {
            Ok(_) => match Student::validate_semester(&semester) {
                Ok(_) => break,
                Err(e) => {
                    eprintln!("Invalid input: {}", e);
                }
            },
            Err(e) => {
                eprintln!("Unable to read input: {}", e);
                continue;
            }
        }
    }

    semester.trim().to_string()
}

fn parse_course_name() -> String {
    let mut course_name = String::new();
    loop {
        println!("Enter course name:");
        match stdin().read_line(&mut course_name) {
            Ok(_) => match Student::validate_course_name(&course_name) {
                Ok(_) => break,
                Err(e) => {
                    eprintln!("Invalid input: {}", e);
                }
            },
            Err(e) => {
                eprintln!("Unable to read input: {}", e);
                continue;
            }
        }
    }

    course_name.trim().to_string()
}

fn parse_student_name() -> String {
    let mut student_name = String::new();
    loop {
        println!("Enter student name:");
        match stdin().read_line(&mut student_name) {
            Ok(_) => match Student::validate_student_name(&student_name) {
                Ok(_) => break,
                Err(e) => {
                    eprintln!("Invalid input: {}", e);
                }
            },
            Err(e) => {
                eprintln!("Unable to read input: {}", e);
                continue;
            }
        }
    }

    student_name.trim().to_string()
}

#[derive(PartialEq, Debug)]
pub struct Student {
    id: i32,
    student_name: String,
    semester: String,
    course_name: String,
}

impl Student {
    pub fn new(id: i32, student_name: String, semester: String, course_name: String) -> Student {
        Student {
            id,
            student_name,
            semester,
            course_name,
        }
    }
    pub fn get_student_id(&self) -> i32 {
        self.id
    }

    pub fn get_student_name(&self) -> &str {
        &self.student_name
    }

    pub fn get_semester(&self) -> &str {
        &self.semester
    }

    pub fn get_course_name(&self) -> &str {
        &self.course_name
    }

    pub fn set_student_name(&mut self, student_name: String) {
        self.student_name = student_name
    }

    pub fn set_semester(&mut self, semester: String) {
        self.semester = semester
    }

    pub fn set_course_name(&mut self, course_name: String) {
        self.course_name = course_name
    }

    pub fn validate_student_name(student_name: &str) -> Result<(), &str> {
        if student_name.trim().is_empty() {
            return Err("student name cannot be empty");
        }

        Ok(())
    }

    pub fn validate_semester(semester: &str) -> Result<(), &str> {
        if semester.trim().is_empty() {
            return Err("semester cannot be empty");
        }

        Ok(())
    }

    pub fn validate_course_name(course_name: &str) -> Result<(), &str> {
        match course_name.trim() {
            "Java" => {}
            "C/C++" => {}
            ".NET" => {}
            _ => return Err("course can only be Java, C/C++ or .NET"),
        }

        Ok(())
    }
}

pub struct StudentManagement {
    students: Vec<Student>,
    student_id: i32,
}

impl StudentManagement {
    pub fn new() -> StudentManagement {
        StudentManagement {
            students: vec![],
            student_id: 0,
        }
    }

    pub fn add(&mut self, student_name: String, semester: String, course_name: String) {
        self.student_id += 1;
        self.students.push(Student {
            id: self.student_id,
            student_name,
            semester,
            course_name,
        });
    }

    pub fn search(&self, student_name: &str) -> Vec<(&str, &str, &str)> {
        let mut students: Vec<(&str, &str, &str)> = self
            .students
            .iter()
            .filter(|student| student.get_student_name().contains(student_name))
            .map(|student| {
                (
                    student.get_student_name(),
                    student.get_semester(),
                    student.get_course_name(),
                )
            })
            .collect();

        students.sort_by(|a, b| a.0.partial_cmp(b.0).unwrap());

        students
    }

    pub fn search_by_id(&self, id: i32) -> Result<usize, &str> {
        let mut idx = None;

        for (index, student) in self.students.iter().enumerate() {
            if student.get_student_id() == id {
                idx = Some(index);
            }
        }

        match idx {
            Some(val) => Ok(val),
            None => Err("Student not found"),
        }
    }

    pub fn delete(&mut self, index: usize) -> Student {
        self.students.swap_remove(index)
    }

    pub fn update(
        &mut self,
        index: usize,
        student_name: String,
        semester: String,
        course_name: String,
    ) {
        let removed_student = Self::delete(self, index);

        self.students.insert(
            index,
            Student {
                id: removed_student.id,
                student_name,
                semester,
                course_name,
            },
        );
    }

    pub fn report(&self) -> Vec<(&str, &str, i32)> {
        let mut map: HashMap<(&str, &str), i32> = HashMap::new();

        for student in self.students.iter() {
            let key = (student.get_student_name(), student.get_course_name());

            map.entry(key)
                .and_modify(|counter| *counter += 1)
                .or_insert(1);
        }

        let mut report: Vec<(&str, &str, i32)> = map
            .iter()
            .map(|(key, value)| (key.0, key.1, *value))
            .collect();

        report.sort_by(|a, b| a.0.partial_cmp(b.0).unwrap());

        report
    }

    pub fn load(&mut self, content: String) {
        for line in content.lines() {
            let words: Vec<&str> = line.split("|").into_iter().collect();

            if words.len() == 3 {
                Self::add(
                    self,
                    words[0].to_string(),
                    words[1].to_string(),
                    words[2].to_string(),
                );
            }
        }
    }

    pub fn save(&self, file_path: &str) -> std::io::Result<()> {
        let mut content = String::new();

        for student in self.students.iter() {
            content += format!(
                "{}|{}|{}\n",
                student.student_name, student.semester, student.course_name
            )
            .as_str();
        }

        fs::write(file_path, content)?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_test() {
        let mut student_management = StudentManagement::new();

        student_management.add(
            String::from("Nguyen Van A"),
            String::from("Semester 1"),
            String::from("Java"),
        );

        assert_eq!(
            vec![Student {
                id: 1,
                student_name: String::from("Nguyen Van A"),
                semester: String::from("Semester 1"),
                course_name: String::from("Java")
            }],
            student_management.students
        );
    }

    #[test]
    fn search_test() {
        let mut student_management = StudentManagement::new();

        student_management.add(
            String::from("Nguyen Van A"),
            String::from("Semester 1"),
            String::from("Java"),
        );
        student_management.add(
            String::from("Nguyen Van A"),
            String::from("Semester 1"),
            String::from("Java"),
        );
        student_management.add(
            String::from("Nguyen Van BC"),
            String::from("Semester 1"),
            String::from(".NET"),
        );
        student_management.add(
            String::from("Nguyen Van C"),
            String::from("Semester 1"),
            String::from("Java"),
        );

        assert_eq!(
            vec![
                ("Nguyen Van BC", "Semester 1", ".NET"),
                ("Nguyen Van C", "Semester 1", "Java")
            ],
            student_management.search("C")
        );
    }

    #[test]
    fn delete_test() {
        let mut student_management = StudentManagement::new();

        student_management.add(
            String::from("Nguyen Van A"),
            String::from("Semester 1"),
            String::from("Java"),
        );
        student_management.add(
            String::from("Nguyen Van A"),
            String::from("Semester 1"),
            String::from("Java"),
        );
        student_management.add(
            String::from("Nguyen Van BC"),
            String::from("Semester 1"),
            String::from(".NET"),
        );
        student_management.add(
            String::from("Nguyen Van C"),
            String::from("Semester 1"),
            String::from("Java"),
        );

        student_management.delete(3);

        assert_eq!(
            vec![
                Student {
                    id: 1,
                    student_name: String::from("Nguyen Van A"),
                    semester: String::from("Semester 1"),
                    course_name: String::from("Java")
                },
                Student {
                    id: 2,
                    student_name: String::from("Nguyen Van A"),
                    semester: String::from("Semester 1"),
                    course_name: String::from("Java")
                },
                Student {
                    id: 3,
                    student_name: String::from("Nguyen Van BC"),
                    semester: String::from("Semester 1"),
                    course_name: String::from(".NET")
                }
            ],
            student_management.students
        );
    }

    #[test]
    fn report_test() {
        let mut student_management = StudentManagement::new();

        student_management.add(
            String::from("Nguyen Van A"),
            String::from("Semester 1"),
            String::from("Java"),
        );
        student_management.add(
            String::from("Nguyen Van A"),
            String::from("Semester 1"),
            String::from("Java"),
        );
        student_management.add(
            String::from("Nguyen Van B"),
            String::from("Semester 1"),
            String::from(".NET"),
        );
        student_management.add(
            String::from("Nguyen Van C"),
            String::from("Semester 1"),
            String::from("Java"),
        );

        assert_eq!(
            vec![
                ("Nguyen Van A", "Java", 2),
                ("Nguyen Van B", ".NET", 1),
                ("Nguyen Van C", "Java", 1)
            ],
            student_management.report()
        );
    }

    #[test]
    fn update_test() {
        let mut student_management = StudentManagement::new();

        student_management.add(
            String::from("Nguyen Van A"),
            String::from("Semester 1"),
            String::from("Java"),
        );
        student_management.add(
            String::from("Nguyen Van A"),
            String::from("Semester 1"),
            String::from("Java"),
        );
        student_management.add(
            String::from("Nguyen Van BC"),
            String::from("Semester 1"),
            String::from(".NET"),
        );
        student_management.add(
            String::from("Nguyen Van C"),
            String::from("Semester 1"),
            String::from("Java"),
        );

        student_management.update(
            2,
            String::from("concak"),
            String::from("concak"),
            String::from("C/C++"),
        );

        assert_eq!(
            vec![
                Student {
                    id: 1,
                    student_name: String::from("Nguyen Van A"),
                    semester: String::from("Semester 1"),
                    course_name: String::from("Java")
                },
                Student {
                    id: 2,
                    student_name: String::from("Nguyen Van A"),
                    semester: String::from("Semester 1"),
                    course_name: String::from("Java")
                },
                Student {
                    id: 3,
                    student_name: String::from("concak"),
                    semester: String::from("concak"),
                    course_name: String::from("C/C++")
                },
                Student {
                    id: 4,
                    student_name: String::from("Nguyen Van C"),
                    semester: String::from("Semester 1"),
                    course_name: String::from("Java")
                }
            ],
            student_management.students
        );
    }
}
