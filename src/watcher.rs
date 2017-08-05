extern crate systray; // Icon in Windows system
extern crate notify_rust; // Notification on UNIX ssytems

use std::{thread, time};
use std::time::Duration;

/// 
///
#[derive(new)]
pub struct watcher {
        courseID: i32,
        grade: Option<i16>,
        sleep_time: Option<u64>,
}

impl watcher {
    fn get_grade(&self) -> i16 {
      // Retrieve the grade for the given ID via HTML parsing
      unimplemented!();
    }

    fn check_grades(mut self) {
    // Check if the previous grade is different
        let new_grade = self.get_grade();
        if self.grade != new_grade {
            self.notify_grade_change();
            self.grade = new_grade;
        }
    }
    
    fn notify_grade_change(&self) {
    // Using any means to notify 
    // the user of the changes of the mark
        unimplemented!();
        // Toast-notifications
        
        // E-mail
        
    }
    
    pub fn run(mut self) {
    /* // Routine
        self.grade = self.get_grade(); // Store the first version of the grade at the start of the program
        loop {
            self.check_grades();
            thread::sleep(Duration::new(self.sleep_time.unwrap_or(100), 0));
        } */
    }
}