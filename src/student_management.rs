use std::{collections::HashMap, error::Error, fs, io::stdin, ops::ControlFlow};

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
                create_student(&mut student_management);
            }
            2 => {
                find_and_sort_student(&student_management);
            }
            3 => {
                if let ControlFlow::Break(_) = update_or_delete_student(&mut student_management) {
                    break;
                }
            }
            4 => {
                report_student(&student_management);
            }
            5 => {
                if let ControlFlow::Break(_) = load_student(&mut student_management) {
                    continue;
                }
            }
            6 => {
                if let ControlFlow::Break(_) = save_student(&student_management) {
                    continue;
                }
            }
            7 => {
                break;
            }
            _ => {}
        };
    }

    Ok(())
}

fn save_student(student_management: &StudentManagement) -> ControlFlow<()> {
    let mut file_path = String::new();
    println!("Enter file path: ");
    match stdin().read_line(&mut file_path) {
        Ok(_) => {}
        Err(e) => {
            eprintln!("Unable to read iput: {}", e);
            return ControlFlow::Break(());
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

    ControlFlow::Continue(())
}

fn load_student(student_management: &mut StudentManagement) -> ControlFlow<()> {
    let mut file_path = String::new();
    println!("Enter file path: ");
    match stdin().read_line(&mut file_path) {
        Ok(_) => {}
        Err(e) => {
            eprintln!("Unable to read iput: {}", e);
            return ControlFlow::Break(());
        }
    }
    let content = match fs::read_to_string(file_path.trim()) {
        Ok(value) => value,
        Err(e) => {
            eprintln!("Cannot read file with filepath {}: {}", &file_path, e);
            return ControlFlow::Break(());
        }
    };
    student_management.load(content);

    ControlFlow::Continue(())
}

fn report_student(student_management: &StudentManagement) {
    for (student_name, course_name, course_count) in student_management.report() {
        println!("{student_name}|{course_name}|{course_count}")
    }
    println!();
}

fn update_or_delete_student(student_management: &mut StudentManagement) -> ControlFlow<()> {
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
            return ControlFlow::Break(());
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
                        input_student_name(),
                        input_semester(),
                        input_course_name(),
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
    ControlFlow::Continue(())
}

fn find_and_sort_student(student_management: &StudentManagement) {
    for (student_name, semester, course_name) in
        student_management.search(input_student_name().get())
    {
        println!("{student_name}|{semester}|{course_name}")
    }
    println!();
}

fn create_student(student_management: &mut StudentManagement) {
    let mut count = 0;

    'outer: loop {
        if count > 1 {
            let mut inner_option = String::new();

            loop {
                println!(
                    "Do you want to continue (Y/N)? Choose Y to continue, N to return main screen."
                );

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

        student_management.add(input_student_name(), input_semester(), input_course_name());
        count += 1;

        println!();
    }
}

fn input_semester() -> Semester {
    loop {
        let mut semester = String::new();

        println!("Enter semester:");
        match stdin().read_line(&mut semester) {
            Ok(_) => match Semester::new(semester) {
                Ok(val) => return val,
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
}

fn input_course_name() -> CourseName {
    loop {
        let mut course_name = String::new();

        println!("Enter course name:");

        match stdin().read_line(&mut course_name) {
            Ok(_) => match CourseName::new(course_name) {
                Ok(val) => return val,
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
}

fn input_student_name() -> StudentName {
    loop {
        let mut student_name = String::new();

        println!("Enter student name:");

        match stdin().read_line(&mut student_name) {
            Ok(_) => match StudentName::new(student_name) {
                Ok(val) => return val,
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
}
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct StudentName(String);

impl StudentName {
    pub fn new(name: String) -> Result<StudentName, &'static str> {
        Self::validate_student_name(&name)?;

        Ok(StudentName(name.trim().to_string()))
    }

    pub fn get(&self) -> &str {
        &self.0
    }

    fn validate_student_name(student_name: &str) -> Result<(), &'static str> {
        if student_name.trim().is_empty() {
            return Err("student name cannot be empty");
        }

        Ok(())
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Semester(String);
impl Semester {
    pub fn new(semester: String) -> Result<Semester, &'static str> {
        Self::validate_semester(&semester)?;

        Ok(Semester(semester.trim().to_string()))
    }

    pub fn get(&self) -> &str {
        &self.0
    }

    pub fn validate_semester(semester: &str) -> Result<(), &'static str> {
        if semester.trim().is_empty() {
            return Err("semester cannot be empty");
        }

        Ok(())
    }
}
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct CourseName(String);
impl CourseName {
    pub fn new(course_name: String) -> Result<CourseName, &'static str> {
        Self::validate_course_name(&course_name)?;

        Ok(CourseName(course_name.trim().to_string()))
    }

    pub fn get(&self) -> &str {
        &self.0
    }

    pub fn validate_course_name(course_name: &str) -> Result<(), &'static str> {
        match course_name.trim() {
            "Java" => {}
            "C/C++" => {}
            ".NET" => {}
            _ => return Err("course can only be Java, C/C++ or .NET"),
        }

        Ok(())
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Student {
    id: i32,
    student_name: StudentName,
    semester: Semester,
    course_name: CourseName,
}

impl Student {
    pub fn student_id(&self) -> i32 {
        self.id
    }

    pub fn student_name(&self) -> &StudentName {
        &self.student_name
    }

    pub fn semester(&self) -> &Semester {
        &self.semester
    }

    pub fn course_name(&self) -> &CourseName {
        &self.course_name
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

    pub fn add(&mut self, student_name: StudentName, semester: Semester, course_name: CourseName) {
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
            .filter(|student| student.student_name().get().contains(student_name))
            .map(|student| {
                (
                    student.student_name().get(),
                    student.semester().get(),
                    student.course_name().get(),
                )
            })
            .collect();

        students.sort_by(|a, b| a.0.partial_cmp(b.0).unwrap());

        students
    }

    pub fn search_by_id(&self, id: i32) -> Result<usize, &str> {
        self.students
            .iter()
            .position(|student| student.student_id() == id)
            .ok_or("Student not found")
    }

    pub fn delete(&mut self, index: usize) -> Student {
        self.students.remove(index)
    }

    pub fn update(
        &mut self,
        index: usize,
        student_name: StudentName,
        semester: Semester,
        course_name: CourseName,
    ) {
        if let Some(student) = self.students.get_mut(index) {
            student.student_name = student_name;
            student.semester = semester;
            student.course_name = course_name;
        }
    }

    pub fn report(&self) -> Vec<(&str, &str, i32)> {
        let mut map: HashMap<(&str, &str), i32> = HashMap::new();

        for student in self.students.iter() {
            let key = (student.student_name().get(), student.course_name().get());

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
        content
            .lines()
            .into_iter()
            .map(|line| line.split("|").collect::<Vec<&str>>())
            .for_each(|words| {
                self.add(
                    StudentName::new(words[0].to_string()).unwrap(),
                    Semester::new(words[1].to_string()).unwrap(),
                    CourseName::new(words[2].to_string()).unwrap(),
                )
            });
    }

    pub fn save(&self, file_path: &str) -> std::io::Result<()> {
        let mut content = String::new();

        self.students.iter().for_each(|student| {
            content += format!(
                "{}|{}|{}\n",
                student.student_name.get(),
                student.semester.get(),
                student.course_name.get()
            )
            .as_str()
        });

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
            StudentName::new(String::from("Nguyen Van A")).unwrap(),
            Semester::new(String::from("Semester 1")).unwrap(),
            CourseName::new(String::from("Java")).unwrap(),
        );

        assert_eq!(
            vec![Student {
                id: 1,
                student_name: StudentName::new(String::from("Nguyen Van A")).unwrap(),
                semester: Semester::new(String::from("Semester 1")).unwrap(),
                course_name: CourseName::new(String::from("Java")).unwrap()
            }],
            student_management.students
        );
    }

    #[test]
    fn search_test() {
        let mut student_management = StudentManagement::new();

        student_management.add(
            StudentName::new(String::from("Nguyen Van A")).unwrap(),
            Semester::new(String::from("Semester 1")).unwrap(),
            CourseName::new(String::from("Java")).unwrap(),
        );
        student_management.add(
            StudentName::new(String::from("Nguyen Van A")).unwrap(),
            Semester::new(String::from("Semester 1")).unwrap(),
            CourseName::new(String::from("Java")).unwrap(),
        );
        student_management.add(
            StudentName::new(String::from("Nguyen Van BC")).unwrap(),
            Semester::new(String::from("Semester 1")).unwrap(),
            CourseName::new(String::from(".NET")).unwrap(),
        );
        student_management.add(
            StudentName::new(String::from("Nguyen Van C")).unwrap(),
            Semester::new(String::from("Semester 1")).unwrap(),
            CourseName::new(String::from("Java")).unwrap(),
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
            StudentName::new(String::from("Nguyen Van A")).unwrap(),
            Semester::new(String::from("Semester 1")).unwrap(),
            CourseName::new(String::from("Java")).unwrap(),
        );
        student_management.add(
            StudentName::new(String::from("Nguyen Van A")).unwrap(),
            Semester::new(String::from("Semester 1")).unwrap(),
            CourseName::new(String::from("Java")).unwrap(),
        );
        student_management.add(
            StudentName::new(String::from("Nguyen Van BC")).unwrap(),
            Semester::new(String::from("Semester 1")).unwrap(),
            CourseName::new(String::from(".NET")).unwrap(),
        );
        student_management.add(
            StudentName::new(String::from("Nguyen Van C")).unwrap(),
            Semester::new(String::from("Semester 1")).unwrap(),
            CourseName::new(String::from("Java")).unwrap(),
        );

        student_management.delete(3);

        assert_eq!(
            vec![
                Student {
                    id: 1,
                    student_name: StudentName::new(String::from("Nguyen Van A")).unwrap(),
                    semester: Semester::new(String::from("Semester 1")).unwrap(),
                    course_name: CourseName::new(String::from("Java")).unwrap()
                },
                Student {
                    id: 2,
                    student_name: StudentName::new(String::from("Nguyen Van A")).unwrap(),
                    semester: Semester::new(String::from("Semester 1")).unwrap(),
                    course_name: CourseName::new(String::from("Java")).unwrap()
                },
                Student {
                    id: 3,
                    student_name: StudentName::new(String::from("Nguyen Van BC")).unwrap(),
                    semester: Semester::new(String::from("Semester 1")).unwrap(),
                    course_name: CourseName::new(String::from(".NET")).unwrap()
                }
            ],
            student_management.students
        );
    }

    #[test]
    fn report_test() {
        let mut student_management = StudentManagement::new();

        student_management.add(
            StudentName::new(String::from("Nguyen Van A")).unwrap(),
            Semester::new(String::from("Semester 1")).unwrap(),
            CourseName::new(String::from("Java")).unwrap(),
        );
        student_management.add(
            StudentName::new(String::from("Nguyen Van A")).unwrap(),
            Semester::new(String::from("Semester 1")).unwrap(),
            CourseName::new(String::from("Java")).unwrap(),
        );
        student_management.add(
            StudentName::new(String::from("Nguyen Van B")).unwrap(),
            Semester::new(String::from("Semester 1")).unwrap(),
            CourseName::new(String::from(".NET")).unwrap(),
        );
        student_management.add(
            StudentName::new(String::from("Nguyen Van C")).unwrap(),
            Semester::new(String::from("Semester 1")).unwrap(),
            CourseName::new(String::from("Java")).unwrap(),
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
            StudentName::new(String::from("Nguyen Van A")).unwrap(),
            Semester::new(String::from("Semester 1")).unwrap(),
            CourseName::new(String::from("Java")).unwrap(),
        );
        student_management.add(
            StudentName::new(String::from("Nguyen Van A")).unwrap(),
            Semester::new(String::from("Semester 1")).unwrap(),
            CourseName::new(String::from("Java")).unwrap(),
        );
        student_management.add(
            StudentName::new(String::from("Nguyen Van BC")).unwrap(),
            Semester::new(String::from("Semester 1")).unwrap(),
            CourseName::new(String::from(".NET")).unwrap(),
        );
        student_management.add(
            StudentName::new(String::from("Nguyen Van C")).unwrap(),
            Semester::new(String::from("Semester 1")).unwrap(),
            CourseName::new(String::from("Java")).unwrap(),
        );

        student_management.update(
            2,
            StudentName::new(String::from("concak")).unwrap(),
            Semester::new(String::from("concak")).unwrap(),
            CourseName::new(String::from("C/C++")).unwrap(),
        );

        assert_eq!(
            vec![
                Student {
                    id: 1,
                    student_name: StudentName::new(String::from("Nguyen Van A")).unwrap(),
                    semester: Semester::new(String::from("Semester 1")).unwrap(),
                    course_name: CourseName::new(String::from("Java")).unwrap()
                },
                Student {
                    id: 2,
                    student_name: StudentName::new(String::from("Nguyen Van A")).unwrap(),
                    semester: Semester::new(String::from("Semester 1")).unwrap(),
                    course_name: CourseName::new(String::from("Java")).unwrap()
                },
                Student {
                    id: 3,
                    student_name: StudentName::new(String::from("concak")).unwrap(),
                    semester: Semester::new(String::from("concak")).unwrap(),
                    course_name: CourseName::new(String::from("C/C++")).unwrap()
                },
                Student {
                    id: 4,
                    student_name: StudentName::new(String::from("Nguyen Van C")).unwrap(),
                    semester: Semester::new(String::from("Semester 1")).unwrap(),
                    course_name: CourseName::new(String::from("Java")).unwrap()
                }
            ],
            student_management.students
        );
    }
}
