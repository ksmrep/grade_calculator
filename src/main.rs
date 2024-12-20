use std::fs::File;
use std::io::{BufRead, BufReader};
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: {} <filename>", args[0]);
        std::process::exit(1);
    }

    let filename = &args[1];
    let file = File::open(filename).expect("Failed to open file");
    let reader = BufReader::new(file);

    let mut courses = Vec::new();
    
    for line in reader.lines() {
        let line = line.expect("Failed to read line");
        let parts: Vec<&str> = line.split(',').map(|s| s.trim()).collect();
        if parts.len() != 3 {
            continue;
        }

        let course_name = parts[0].to_string();
        let credits: i32 = parts[1].parse().unwrap_or(0);
        let grade = parts[2].to_string();
        let usa_gpa = match grade.as_str() {
            "A+" | "A0" => 4.0,
            "A-" => 3.7,
            "B+" => 3.3,
            "B0" => 3.0,
            "B-" => 2.7,
            "C+" => 2.3,
            "C0" => 2.0,
            "C-" => 1.7,
            "D+" => 1.3,
            "D0" => 1.0,
            "D-" => 0.7,
            "F"  => 0.0,
            _ => 0.0,
        };
        let gpa = match grade.as_str() {
            "A+" => 4.3,
            "A0" => 4.0,
            "A-" => 3.7,
            "B+" => 3.3,
            "B0" => 3.0,
            "B-" => 2.7,
            "C+" => 2.3,
            "C0" => 2.0,
            "C-" => 1.7,
            "D+" => 1.3,
            "D0" => 1.0,
            "D-" => 0.7,
            "F"  => 0.0,
            _ => 0.0,
        };
        
        courses.push((course_name, credits, grade, usa_gpa, gpa));
    }
    let available_grades: Vec<String> = vec!["A+", "A0", "A-", "B+", "B0", "B-", "C+", "C0", "C-", "D+", "D0", "D-", "F"].into_iter().map(String::from).collect();
    
    let mut total_weighted_score: f32 = 0.0;
    let mut total_weighted_us_score: f32 = 0.0;
    let mut total_credits: i32 = 0;

    for (course_name, credits, grade, usa_gpa, gpa) in &courses {
        println!("Course: {}, Credits: {}, US GPA: {}, KR GPA: {}", course_name, credits, usa_gpa, gpa);
        if available_grades.contains(grade) {
            total_weighted_us_score += usa_gpa * (*credits as f32);
            total_weighted_score += gpa * (*credits as f32);
            total_credits += *credits as i32;
        }
    }

    if total_credits > 0 {
        let calculated_usa_gpa = total_weighted_us_score / total_credits as f32;
        let calculated_gpa = total_weighted_score / total_credits as f32;
        println!("Calculated US GPA: {:.2}", calculated_usa_gpa);
        println!("Calculated KR GPA: {:.2}", calculated_gpa);
    } else {
        println!("No courses found.");
    }

}
