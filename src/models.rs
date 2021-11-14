use iron_planet::keys;
use pyo3::exceptions;
use pyo3::prelude::*;

#[pyclass]
pub struct Department {
    pub(crate) department: iron_planet::models::Department,
}

#[pymethods]
impl Department {
    #[getter]
    fn id(&self) -> PyResult<keys::DepartmentKey> {
        Ok(self.department.id)
    }

    #[getter]
    fn name(&self) -> PyResult<String> {
        Ok(self.department.name.clone())
    }

    #[getter]
    fn description(&self) -> PyResult<Option<String>> {
        Ok(self.department.description.clone())
    }

    #[getter]
    fn building(&self) -> PyResult<Option<Building>> {
        Ok(self
            .department
            .get_building()
            .map_err(|e| exceptions::PyException::new_err(format!("{:?}", e)))?
            .map(|building| Building { building }))
    }

    #[getter]
    fn courses(&self) -> PyResult<Vec<Course>> {
        Ok(self
            .department
            .get_courses()
            .map_err(|e| exceptions::PyException::new_err(format!("{:?}", e)))?
            .into_iter()
            .map(|course| Course { course })
            .collect())
    }
}

#[pyclass]
pub struct Building {
    pub(crate) building: iron_planet::models::Building,
}

#[pymethods]
impl Building {
    #[getter]
    fn id(&self) -> PyResult<keys::BuildingKey> {
        Ok(self.building.id)
    }

    #[getter]
    fn name(&self) -> PyResult<String> {
        Ok(self.building.name.clone())
    }

    #[getter]
    fn abbreviation(&self) -> PyResult<String> {
        Ok(self.building.abbreviation.clone())
    }

    #[getter]
    fn places(&self) -> PyResult<Vec<Place>> {
        Ok(self
            .building
            .get_rooms()
            .map_err(|e| exceptions::PyException::new_err(format!("{:?}", e)))?
            .into_iter()
            .map(|place| Place { place })
            .collect())
    }
}

#[pyclass]
pub struct Place {
    pub(crate) place: iron_planet::models::Place,
}

#[pymethods]
impl Place {
    #[getter]
    fn id(&self) -> PyResult<keys::PlaceKey> {
        Ok(self.place.id)
    }

    #[getter]
    fn name(&self) -> PyResult<String> {
        Ok(self.place.name.clone())
    }

    #[getter]
    fn floor(&self) -> PyResult<i8> {
        Ok(self.place.floor)
    }

    #[getter]
    fn picture(&self) -> PyResult<Option<String>> {
        Ok(self.place.picture.clone())
    }

    #[getter]
    fn picture_cover(&self) -> PyResult<Option<String>> {
        Ok(self.place.picture_cover.clone())
    }

    fn is_room(&self) -> PyResult<bool> {
        Ok(matches!(
            self.place.variant,
            iron_planet::models::PlaceVariant::Room(_)
        ))
    }

    #[getter]
    fn building(&self) -> PyResult<Option<Building>> {
        Ok(self
            .place
            .get_building()
            .map_err(|e| exceptions::PyException::new_err(format!("{:?}", e)))?
            .map(|building| Building { building }))
    }

    #[getter]
    fn capacity(&self) -> PyResult<Option<u16>> {
        if let iron_planet::models::PlaceVariant::Room(room) = &self.place.variant {
            Ok(room.capacity)
        } else {
            Ok(None)
        }
    }

    #[getter]
    fn door_number(&self) -> PyResult<Option<u16>> {
        if let iron_planet::models::PlaceVariant::Room(room) = &self.place.variant {
            Ok(room.door_number)
        } else {
            Ok(None)
        }
    }

    #[getter]
    fn description(&self) -> PyResult<Option<String>> {
        if let iron_planet::models::PlaceVariant::Room(room) = &self.place.variant {
            Ok(room.description.clone())
        } else {
            Ok(None)
        }
    }

    #[getter]
    fn equipment(&self) -> PyResult<Option<String>> {
        if let iron_planet::models::PlaceVariant::Room(room) = &self.place.variant {
            Ok(room.equipment.clone())
        } else {
            Ok(None)
        }
    }

    #[getter]
    fn department(&self) -> PyResult<Option<Department>> {
        if let iron_planet::models::PlaceVariant::Room(room) = &self.place.variant {
            Ok(room
                .get_department()
                .map_err(|e| exceptions::PyException::new_err(format!("{:?}", e)))?
                .map(|department| Department { department }))
        } else {
            Ok(None)
        }
    }
}

#[pyclass]
pub struct Course {
    pub(crate) course: iron_planet::models::Course,
}

#[pymethods]
impl Course {
    #[getter]
    fn id(&self) -> PyResult<keys::CourseKey> {
        Ok(self.course.id)
    }

    #[getter]
    fn abbreviation(&self) -> PyResult<String> {
        Ok(self.course.abbreviation.clone())
    }

    #[getter]
    fn name(&self) -> PyResult<String> {
        Ok(self.course.name.clone())
    }

    #[getter]
    fn degree(&self) -> PyResult<String> {
        todo!()
    }

    #[getter]
    fn department(&self) -> PyResult<Option<Department>> {
        Ok(self
            .course
            .get_department()
            .map_err(|e| exceptions::PyException::new_err(format!("{:?}", e)))?
            .map(|department| Department { department }))
    }
}

#[pyclass]
pub struct Class {
    pub(crate) klass: iron_planet::models::Class,
}

#[pymethods]
impl Class {
    #[getter]
    fn id(&self) -> PyResult<keys::ClassKey> {
        Ok(self.klass.id)
    }

    #[getter]
    fn name(&self) -> PyResult<String> {
        Ok(self.klass.name.clone())
    }

    #[getter]
    fn abbreviation(&self) -> PyResult<String> {
        Ok(self.klass.abbreviation.clone())
    }

    #[getter]
    fn credits(&self) -> PyResult<u32> {
        Ok(self.klass.credits)
    }

    #[getter]
    fn instances(&self) -> PyResult<Vec<ClassInstance>> {
        Ok(self
            .klass
            .get_instances()
            .map_err(|e| exceptions::PyException::new_err(format!("{:?}", e)))?
            .into_iter()
            .map(|instance| ClassInstance { instance })
            .collect())
    }

    #[getter]
    fn department(&self) -> PyResult<Option<Department>> {
        Ok(self
            .klass
            .get_department()
            .map_err(|e| exceptions::PyException::new_err(format!("{:?}", e)))?
            .map(|department| Department { department }))
    }
}

#[pyclass]
pub struct ClassInstance {
    pub(crate) instance: iron_planet::models::ClassInstance,
}

#[pymethods]
impl ClassInstance {
    #[getter]
    fn id(&self) -> PyResult<keys::ClassInstanceKey> {
        Ok(self.instance.id)
    }

    #[getter]
    fn year(&self) -> PyResult<u32> {
        Ok(self.instance.year)
    }

    #[getter]
    fn period(&self) -> PyResult<u32> {
        todo!()
    }

    #[getter]
    fn information(&self) -> PyResult<u32> {
        todo!()
    }

    #[getter]
    fn avg_grade(&self) -> PyResult<Option<f32>> {
        Ok(self.instance.avg_grade.clone())
    }

    #[getter]
    fn enrollments(&self) -> PyResult<Vec<Enrollment>> {
        Ok(self
            .instance
            .get_enrollments()
            .map_err(|e| exceptions::PyException::new_err(format!("{:?}", e)))?
            .into_iter()
            .map(|enrollment| Enrollment { enrollment })
            .collect())
    }

    #[getter]
    fn shifts(&self) -> PyResult<Vec<ClassShift>> {
        Ok(self
            .instance
            .get_shifts()
            .map_err(|e| exceptions::PyException::new_err(format!("{:?}", e)))?
            .into_iter()
            .map(|shift| ClassShift { shift })
            .collect())
    }

    #[getter]
    fn department(&self) -> PyResult<Option<Department>> {
        Ok(self
            .instance
            .get_department()
            .map_err(|e| exceptions::PyException::new_err(format!("{:?}", e)))?
            .map(|department| Department { department }))
    }
}

#[pyclass]
pub struct ClassShift {
    pub(crate) shift: iron_planet::models::ClassShift,
}

#[pymethods]
impl ClassShift {
    #[getter]
    fn id(&self) -> PyResult<keys::ShiftKey> {
        Ok(self.shift.id)
    }

    #[getter]
    fn number(&self) -> PyResult<u16> {
        Ok(self.shift.number)
    }

    #[getter]
    fn shift_type(&self) -> PyResult<u16> {
        todo!()
    }

    #[getter]
    fn teachers(&self) -> PyResult<Vec<Teacher>> {
        Ok(self
            .shift
            .get_teachers()
            .map_err(|e| exceptions::PyException::new_err(format!("{:?}", e)))?
            .into_iter()
            .map(|teacher| Teacher { teacher })
            .collect())
    }

    #[getter]
    fn instances(&self) -> PyResult<Vec<ClassShiftInstance>> {
        Ok(self
            .shift
            .instances
            .iter()
            .map(|instance| ClassShiftInstance {
                instance: instance.clone(),
            })
            .collect())
    }
}

#[pyclass]
pub struct ClassShiftInstance {
    pub(crate) instance: iron_planet::models::ClassShiftInstance,
}

#[pymethods]
impl ClassShiftInstance {
    #[getter]
    fn weekday(&self) -> PyResult<keys::StudentKey> {
        todo!()
    }

    #[getter]
    fn start(&self) -> PyResult<u16> {
        Ok(self.instance.start)
    }

    #[getter]
    fn duration(&self) -> PyResult<u16> {
        Ok(self.instance.duration)
    }

    #[getter]
    fn room(&self) -> PyResult<Option<Place>> {
        Ok(self
            .instance
            .get_place()
            .map_err(|e| exceptions::PyException::new_err(format!("{:?}", e)))?
            .map(|place| Place { place }))
    }
}

#[pyclass]
pub struct Student {
    pub(crate) student: iron_planet::models::Student,
}

#[pymethods]
impl Student {
    #[getter]
    fn id(&self) -> PyResult<keys::StudentKey> {
        Ok(self.student.id)
    }

    #[getter]
    fn name(&self) -> PyResult<String> {
        Ok(self.student.name.clone())
    }

    #[getter]
    fn abbreviation(&self) -> PyResult<Option<String>> {
        Ok(self.student.abbreviation.clone())
    }

    #[getter]
    fn number(&self) -> PyResult<u32> {
        Ok(self.student.number)
    }

    #[getter]
    fn first_year(&self) -> PyResult<Option<u32>> {
        Ok(self.student.first_year.clone())
    }

    #[getter]
    fn last_year(&self) -> PyResult<Option<u32>> {
        Ok(self.student.first_year.clone())
    }

    #[getter]
    fn avg_grade(&self) -> PyResult<Option<u32>> {
        Ok(self.student.avg_grade.clone())
    }

    #[getter]
    fn url(&self) -> PyResult<String> {
        Ok(self.student.url.clone())
    }

    #[getter]
    fn enrollments(&self) -> PyResult<Vec<Enrollment>> {
        Ok(self
            .student
            .get_enrollments()
            .map_err(|e| exceptions::PyException::new_err(format!("{:?}", e)))?
            .into_iter()
            .map(|enrollment| Enrollment { enrollment })
            .collect())
    }

    #[getter]
    fn shifts(&self) -> PyResult<Vec<ClassShift>> {
        Ok(self
            .student
            .get_shifts()
            .map_err(|e| exceptions::PyException::new_err(format!("{:?}", e)))?
            .into_iter()
            .map(|shift| ClassShift { shift })
            .collect())
    }

    #[getter]
    fn course(&self) -> PyResult<Option<Course>> {
        Ok(self
            .student
            .get_course()
            .map_err(|e| exceptions::PyException::new_err(format!("{:?}", e)))?
            .map(|course| Course { course }))
    }
}

#[pyclass]
pub struct Teacher {
    pub(crate) teacher: iron_planet::models::Teacher,
}

#[pymethods]
impl Teacher {
    #[getter]
    fn id(&self) -> PyResult<keys::TeacherKey> {
        Ok(self.teacher.id)
    }

    #[getter]
    fn name(&self) -> PyResult<String> {
        Ok(self.teacher.name.clone())
    }

    #[getter]
    fn abbreviation(&self) -> PyResult<Option<String>> {
        Ok(self.teacher.abbreviation.clone())
    }

    #[getter]
    fn first_year(&self) -> PyResult<Option<u32>> {
        Ok(self.teacher.first_year.clone())
    }

    #[getter]
    fn last_year(&self) -> PyResult<Option<u32>> {
        Ok(self.teacher.first_year.clone())
    }

    #[getter]
    fn phone(&self) -> PyResult<Option<String>> {
        Ok(self.teacher.phone.clone())
    }

    #[getter]
    fn email(&self) -> PyResult<Option<String>> {
        Ok(self.teacher.email.clone())
    }

    #[getter]
    fn thumb(&self) -> PyResult<Option<String>> {
        Ok(self.teacher.thumb.clone())
    }

    #[getter]
    fn rank(&self) -> PyResult<Option<String>> {
        Ok(self.teacher.rank.clone())
    }

    #[getter]
    fn url(&self) -> PyResult<String> {
        Ok(self.teacher.url.clone())
    }

    #[getter]
    fn departments(&self) -> PyResult<Vec<Department>> {
        Ok(self
            .teacher
            .get_departments()
            .map_err(|e| exceptions::PyException::new_err(format!("{:?}", e)))?
            .into_iter()
            .map(|department| Department { department })
            .collect())
    }

    #[getter]
    fn shifts(&self) -> PyResult<Vec<ClassShift>> {
        Ok(self
            .teacher
            .get_shifts()
            .map_err(|e| exceptions::PyException::new_err(format!("{:?}", e)))?
            .into_iter()
            .map(|shift| ClassShift { shift })
            .collect())
    }
}

#[pyclass]
pub struct Enrollment {
    pub(crate) enrollment: iron_planet::models::Enrollment,
}

#[pymethods]
impl Enrollment {
    #[getter]
    fn id(&self) -> PyResult<keys::EnrollmentKey> {
        Ok(self.enrollment.id)
    }

    #[getter]
    fn attendance(&self) -> PyResult<Option<bool>> {
        Ok(self.enrollment.attendance)
    }

    #[getter]
    fn attendance_date(&self) -> PyResult<Option<String>> {
        Ok(self.enrollment.attendance_date.clone())
    }

    #[getter]
    fn normal_grade(&self) -> PyResult<Option<u8>> {
        Ok(self.enrollment.normal_grade)
    }

    #[getter]
    fn normal_grade_date(&self) -> PyResult<Option<String>> {
        Ok(self.enrollment.normal_grade_date.clone())
    }

    #[getter]
    fn recourse_grade(&self) -> PyResult<Option<u8>> {
        Ok(self.enrollment.recourse_grade)
    }

    #[getter]
    fn recourse_grade_date(&self) -> PyResult<Option<String>> {
        Ok(self.enrollment.recourse_grade_date.clone())
    }

    #[getter]
    fn special_grade(&self) -> PyResult<Option<u8>> {
        Ok(self.enrollment.special_grade)
    }

    #[getter]
    fn special_grade_date(&self) -> PyResult<Option<String>> {
        Ok(self.enrollment.special_grade_date.clone())
    }

    #[getter]
    fn improvement_grade(&self) -> PyResult<Option<u8>> {
        Ok(self.enrollment.improvement_grade)
    }

    #[getter]
    fn improvement_grade_date(&self) -> PyResult<Option<String>> {
        Ok(self.enrollment.improvement_grade_date.clone())
    }

    #[getter]
    fn approved(&self) -> PyResult<Option<bool>> {
        Ok(self.enrollment.approved.clone())
    }

    #[getter]
    fn grade(&self) -> PyResult<Option<u8>> {
        Ok(self.enrollment.grade.clone())
    }

    #[getter]
    fn student(&self) -> PyResult<Student> {
        Ok(Student {
            student: self
                .enrollment
                .get_student()
                .map_err(|e| exceptions::PyException::new_err(format!("{:?}", e)))?,
        })
    }

    #[getter]
    fn class_instance(&self) -> PyResult<ClassInstance> {
        Ok(ClassInstance {
            instance: self
                .enrollment
                .get_class_instance()
                .map_err(|e| exceptions::PyException::new_err(format!("{:?}", e)))?,
        })
    }
}
