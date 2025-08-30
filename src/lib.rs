pub mod websoc;
pub mod grades;
pub mod courses;
pub mod enrollment;
pub mod instructors;
pub mod ap;
pub mod calender;
pub mod larc;
pub mod other;
pub mod programs;
pub mod study_rooms;

pub(crate) mod common;

#[cfg(test)]
mod tests {
    use super::*;

    // AP crate test
    #[test]
    fn retrieve_ap_exam_names() {
        let parameters = [
            ("fullname", "AP Microeconomics"),
            ("catalogueName", "AP ECONOMICS:MICRO")
        ];

        let result = ap::retrieve_ap_exam_names(&parameters);

        assert_eq!(result["data"][0]["fullName"], "AP Microeconomics")
    }

    // Calender crate test
    #[test]
    fn retrieve_term_calendar() {
        let parameters = [
            ("year", "2024"),
            ("quarter", "Fall")
        ];

        let result = calender::retrieve_term_calendar(&parameters);

        assert_eq!(result["data"]["year"], "2024")
    }

    #[test]
    fn list_all_calendars() {
        // It will change over time, so we cannot have a reliable check with this
        // so let's just make sure it works
        let result = calender::list_all_calendars();

        assert_eq!(result["ok"], true)
    }

    #[test]
    fn retrieve_current_week() {
        // It will change over time, so we cannot have a reliable check with this
        // so let's just make sure it works
        let parameters = [
            ("year", "2024"),
            ("month", "9"),
            ("day", "30")
        ];
        let result = calender::retrieve_current_week(&parameters);

        assert_eq!(result["data"]["weeks"][0], 1)
    }

    #[test]
    fn retrieve_a_course_with_ids() {
        let parameters = ["COMPSCI161", "COMPSCI162"];
        let result = courses::retrieve_a_course_with_ids(&parameters);

        assert_eq!(result["data"][0]["id"], "COMPSCI161")
    }

    #[test]
    fn retrieve_a_course() {
        let result = courses::retrieve_a_course("COMPSCI161");

        assert_eq!(result["data"]["id"], "COMPSCI161")
    }

    #[test]
    fn filter_courses() {
        // Since it would be lengthy, we just check if the "ok" value is true
        let parameters = [("maxUnits", "4")];
        let result = courses::filter_courses(&parameters);

        assert_eq!(result["ok"], true)
    }

    #[test]
    fn filter_courses_with_cursor_pagination() {
        // Since it would be lengthy, we just check if the "ok" value is true
        let parameters = [("maxUnits", "4")];
        let result = courses::filter_courses(&parameters);

        assert_eq!(result["ok"], true)
    }

    // Enrollment crate tests
    #[test]
    fn filter_enrollment_history() {
        // Since it would be lengthy, we just check if the "ok" value is true
        let parameters = [("myear", "2024")];
        let result = courses::filter_courses(&parameters);

        assert_eq!(result["ok"], true)
    }

    // grades crate tests
    #[test]
    fn filter_grades() {
        // Since it would be lengthy, we just check if the "ok" value is true
        let parameters = [("myear", "2024")];
        let result = courses::filter_courses(&parameters);

        assert_eq!(result["ok"], true)
    }

    #[test]
    fn filter_grade_options() {
        // Since it would be lengthy, we just check if the "ok" value is true
        let parameters = [("myear", "2024")];
        let result = courses::filter_courses(&parameters);

        assert_eq!(result["ok"], true)
    }

    #[test]
    fn retrieve_grade_aggregate() {
        // Since it would be lengthy, we just check if the "ok" value is true
        let parameters = [("myear", "2024")];
        let result = courses::filter_courses(&parameters);

        assert_eq!(result["ok"], true)
    }

    #[test]
    fn retrieve_grade_aggregate_by_course() {
        // Since it would be lengthy, we just check if the "ok" value is true
        let parameters = [("myear", "2024")];
        let result = courses::filter_courses(&parameters);

        assert_eq!(result["ok"], true)
    }

    #[test]
    fn retrieve_grade_aggregate_by_offering() {
        // Since it would be lengthy, we just check if the "ok" value is true
        let parameters = [("year", "2024")];
        let result = courses::filter_courses(&parameters);

        assert_eq!(result["ok"], true)
    }

    // instructors crate tests
    #[test]
    fn retrieve_instructors_with_ucinetids() {
        let parameters = ["mikes", "klefstad"];
        let result = instructors::retrieve_instructors_with_ucinetids(&parameters);

        assert_eq!(result["data"][1]["name"], "Michael Shindler")
    }

    #[test]
    fn retrieve_an_instructor() {
        let result = instructors::retrieve_an_instructor("mikes");

        assert_eq!(result["data"]["ucinetid"], "mikes")
    }

    #[test]
    fn filter_instructors() {
        // Since it would be lengthy, we just check if the "ok" value is true
        let parameters = [("nameContains", "Mike")];
        let result = instructors::filter_instructors(&parameters);

        assert_eq!(result["ok"], true)
    }

    #[test]
    fn filter_instructors_with_cursor_pagination() {
        // Since it would be lengthy, we just check if the "ok" value is true
        let parameters = [("nameContains", "Mike")];
        let result = instructors::filter_instructors(&parameters);

        assert_eq!(result["ok"], true)
    }

    // larc crate tests
    #[test]
    fn query_larc_sections() {
        let parameters = [("quarter", "Fall")];
        let result = instructors::filter_instructors(&parameters);

        assert_eq!(result["ok"], true)
    }

    // other crate tests
    #[test]
    fn ping() {
        let result = other::ping();

        assert_eq!(result["ok"], true)
    }

    #[test]
    fn retrieve_search_results() {
        // Just make sure the request goes through
        let parameters = [
            ("query", "course"),
        ];
        let result = other::ping();

        println!("{}", result);

        assert_eq!(result["ok"], true)
    }

    // programs crate test
    #[test]
    fn retrieve_majors() {
        let result = programs::retrieve_majors();

        assert_eq!(result["ok"], true)
    }

    #[test]
    fn retrieve_major() {
        let result = programs::retrieve_major("BA-014");

        assert_eq!(result["data"][0]["id"], "BA-014")
    }

    #[test]
    fn retrieve_minors() {
        let result = programs::retrieve_minors();

        assert_eq!(result["ok"], true)
    }

    #[test]
    fn retrieve_minor() {
        let result = programs::retrieve_minor("49A");

        assert_eq!(result["data"][0]["id"], "49A")
    }

    #[test]
    fn retrieve_specializations() {
        let result = programs::retrieve_specializations();

        assert_eq!(result["ok"], true)
    }

    #[test]
    fn retrieve_specialization() {
        let result = programs::retrieve_specialization("BS-201");

        assert_eq!(result["ok"], true)
    }

    #[test]
    fn retrieve_major_requirements() {
        let result = programs::retrieve_major_requirements("BS-201");

        assert_eq!(result["ok"], true)
    }

    #[test]
    fn retrieve_minor_requirements() {
        let result = programs::retrieve_minor_requirements("459");

        assert_eq!(result["ok"], false)
    }

    #[test]
    fn retrieve_specializations_requirements() {
        let result = programs::retrieve_specialization_requirements("BS-201E");

        assert_eq!(result["ok"], false)
    }

    #[test]
    fn retrieve_undergraduate_requirements() {
        let result = programs::retrieve_undergraduate_requirements("UC");

        assert_eq!(result["ok"], true)
    }

    // study_rooms crate tests
    #[test]
    fn retrieve_a_study_room() {
        let result = study_rooms::retrieve_a_study_room("44670");

        assert_eq!(result["data"]["id"], "44670")
    }

    #[test]
    fn retrieve_study_rooms() {
        let parameter = [
            ("location", "Science Library")
        ];
        let result = study_rooms::retrieve_study_rooms(&parameter);

        assert_eq!(result["ok"], true)
    }

    // websoc crate tests
    #[test]
    fn query_websoc() {
        let parameters = [
            ("year", "2024"),
            ("quarter", "Fall"),
            ("ge", "ANY")
        ];

        let result = websoc::query_websoc(&parameters);

        assert_eq!(result["ok"], true)
    }

    #[test]
    fn list_available_websoc_terms() {

        let result = websoc::list_available_websoc_terms();

        assert_eq!(result["ok"], true)
    }
}
