#[macro_use]
extern crate serde;
use candid::{Decode, Encode};
use ic_cdk::api::{time, caller};
use ic_stable_structures::memory_manager::{MemoryId, MemoryManager, VirtualMemory};
use ic_stable_structures::{BoundedStorable, Cell, DefaultMemoryImpl, StableBTreeMap, Storable};
use std::{borrow::Cow, cell::RefCell};

type Memory = VirtualMemory<DefaultMemoryImpl>;
type IdCell = Cell<u64, Memory>;

#[derive(candid::CandidType, Clone, Serialize, Deserialize, Default)]
struct Course {
    id: u64,
    creator_principal: String,
    title: String,
    description: String,
    lessons: Vec<u64>, // Change lessons type to store lesson IDs
    created_at: u64,
    updated_at: Option<u64>,
}

#[derive(candid::CandidType, Clone, Serialize, Deserialize, Default)]
struct Lesson {
    id: u64,
    course_id: u64,
    title: String,
    content: String,
    created_at: u64,
    updated_at: Option<u64>,
}



#[derive(candid::CandidType, Clone, Serialize, Deserialize, Default)]
struct Certificate {
    id: u64,
    course_id: u64,
    user_id: u64,
    issue_date: u64,
}

#[derive(candid::CandidType, Clone, Serialize, Deserialize, Default)]
struct User {
    id: u64,
    username: String,
    public_key: String,
}

// Storable and BoundedStorable implementations for new structs

impl Storable for Course {
    fn to_bytes(&self) -> std::borrow::Cow<[u8]> {
        Cow::Owned(Encode!(self).unwrap())
    }

    fn from_bytes(bytes: std::borrow::Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
    }
}

impl BoundedStorable for Course {
    const MAX_SIZE: u32 = 1024;
    const IS_FIXED_SIZE: bool = false;
}

impl Storable for Lesson {
    fn to_bytes(&self) -> std::borrow::Cow<[u8]> {
        Cow::Owned(Encode!(self).unwrap())
    }

    fn from_bytes(bytes: std::borrow::Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
    }
}

impl BoundedStorable for Lesson {
    const MAX_SIZE: u32 = 1024;
    const IS_FIXED_SIZE: bool = false;
}

impl Storable for Certificate {
    fn to_bytes(&self) -> std::borrow::Cow<[u8]> {
        Cow::Owned(Encode!(self).unwrap())
    }

    fn from_bytes(bytes: std::borrow::Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
    }
}

impl BoundedStorable for Certificate {
    const MAX_SIZE: u32 = 1024;
    const IS_FIXED_SIZE: bool = false;
}

impl Storable for User {
    fn to_bytes(&self) -> std::borrow::Cow<[u8]> {
        Cow::Owned(Encode!(self).unwrap())
    }

    fn from_bytes(bytes: std::borrow::Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
    }
}

impl BoundedStorable for User {
    const MAX_SIZE: u32 = 1024;
    const IS_FIXED_SIZE: bool = false;
}


#[derive(candid::CandidType, Clone, Serialize, Deserialize, Default)]
struct LessonPayload {
    title: String,
    content: String,
}

#[derive(candid::CandidType, Clone, Serialize, Deserialize, Default)]
struct CoursePayload {
    title: String,
    description: String,
}

// Storage for new structs

thread_local! {
    static MEMORY_MANAGER: RefCell<MemoryManager<DefaultMemoryImpl>> = RefCell::new(
        MemoryManager::init(DefaultMemoryImpl::default())
    );

    static ID_COUNTER: RefCell<IdCell> = RefCell::new(
        IdCell::init(MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(0))), 0)
            .expect("Cannot create a counter")
    );

    static COURSE_STORAGE: RefCell<StableBTreeMap<u64, Course, Memory>> =
        RefCell::new(StableBTreeMap::init(
            MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(2)))
    ));

    static LESSON_STORAGE: RefCell<StableBTreeMap<u64, Lesson, Memory>> =
        RefCell::new(StableBTreeMap::init(
            MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(3)))
    ));

    static CERTIFICATE_STORAGE: RefCell<StableBTreeMap<u64, Certificate, Memory>> =
        RefCell::new(StableBTreeMap::init(
            MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(4)))
    ));

    static USER_STORAGE: RefCell<StableBTreeMap<u64, User, Memory>> =
        RefCell::new(StableBTreeMap::init(
            MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(5)))
    ));
}

// Query and update functions for new structs

#[ic_cdk::query]
fn get_course(id: u64) -> Result<Course, Error> {
    match _get_course(&id) {
        Some(course) => Ok(course),
        None => Err(Error::NotFound {
            msg: format!("a course with id={} not found", id),
        }),
    }
}

fn is_course_creator(course: &Course) -> Result<(), Error> {
    if course.creator_principal != caller().to_string(){
        return Err(Error::NotCreator)
    }else{
        Ok(())
    }
}

#[ic_cdk::update]
fn add_course(course: CoursePayload) -> Option<Course> {
    let id = ID_COUNTER
        .with(|counter| {
            let current_value = *counter.borrow().get();
            counter.borrow_mut().set(current_value + 1)
        })
        .expect("cannot increment id counter");
    let course = Course {
        id,
        creator_principal: caller().to_string(),
        title: course.title,
        description: course.description,
        lessons: Vec::new(),
        created_at: time(),
        updated_at: None,
    };
    COURSE_STORAGE.with(|service| service.borrow_mut().insert(course.id, course.clone()));
    Some(course)
}

#[ic_cdk::update]
fn update_course(id: u64, payload: CoursePayload) -> Result<Course, Error> {
    let course = _get_course(&id).ok_or_else(|| Error::NotFound {
        msg: format!("course with id={} not found", id),
    })?;

    is_course_creator(&course)?;

    // Create an updated course with the new information
    let updated_course = Course {
        id: course.id,
        creator_principal: course.creator_principal,
        title: payload.title,
        description: payload.description,
        lessons: course.lessons.clone(),
        created_at: course.created_at,
        updated_at: Some(time()),
    };

    // Replace the existing course with the updated version
    COURSE_STORAGE.with(|service| {
        service
            .borrow_mut()
            .insert(course.id, updated_course.clone());
    });

    Ok(updated_course)
}

#[ic_cdk::update]
fn delete_course(id: u64) -> Result<(), Error> {
    let mut _course = _get_course(&id).ok_or_else(|| Error::NotFound {
        msg: format!("course with id={} not found", id),
    })?;

    is_course_creator(&_course)?;

     // Optionally, you may want to delete associated lessons, certificates, or user enrollments
    _course.lessons.iter_mut().for_each(|lesson_id| {
        let _ = delete_lesson(lesson_id.clone());
    });

    // Delete the course
    COURSE_STORAGE.with(|service| service.borrow_mut().remove(&id));
    Ok(())
}

#[ic_cdk::query]
fn get_lesson(id: u64) -> Result<Lesson, Error> {
    match _get_lesson(&id) {
        Some(lesson) => Ok(lesson),
        None => Err(Error::NotFound {
            msg: format!("a lesson with id={} not found", id),
        }),
    }
}

#[ic_cdk::update]
fn add_lesson(lesson: LessonPayload, course_id: u64) -> Result<(), Error> {
    let course = _get_course(&course_id).ok_or_else(|| Error::NotFound {
        msg: format!("course with id={} not found", course_id),
    })?;

    is_course_creator(&course)?;

    let id = ID_COUNTER
        .with(|counter| {
            let current_value = *counter.borrow().get();
            counter.borrow_mut().set(current_value + 1)
        })
        .expect("cannot increment id counter");
    let new_lesson = Lesson {
        id,
        course_id,
        title: lesson.title,
        content: lesson.content,
        created_at: time(),
        updated_at: None,
    };
    LESSON_STORAGE.with(|service| service.borrow_mut().insert(new_lesson.id, new_lesson.clone()));

    // Create an updated course with the new lesson
    let updated_course = Course {
        id: course.id,
        creator_principal: course.creator_principal,
        title: course.title.clone(),
        description: course.description.clone(),
        lessons: {
            let mut lessons = course.lessons.clone();
            lessons.push(id);
            lessons
        },
        created_at: course.created_at,
        updated_at: Some(time()),
    };

    // Replace the existing course with the updated version
    COURSE_STORAGE.with(|service| {
        service
            .borrow_mut()
            .insert(course.id, updated_course.clone());
    });

    Ok(())
}

#[ic_cdk::update]
fn update_lesson(id: u64, payload: LessonPayload) -> Result<Lesson, Error> {
    let lesson = _get_lesson(&id).ok_or_else(|| Error::NotFound {
        msg: format!("lesson with id={} not found", id),
    })?;

    let course = _get_course(&lesson.course_id).ok_or_else(|| Error::NotFound {
        msg: format!("course with id={} not found", lesson.course_id),
    })?;

    is_course_creator(&course)?;

    // Create an updated lesson with the new information
    let updated_lesson = Lesson {
        id: lesson.id,
        course_id: lesson.course_id,
        title: payload.title,
        content: payload.content,
        created_at: lesson.created_at,
        updated_at: Some(time()),
    };

    // Replace the existing lesson with the updated version
    LESSON_STORAGE.with(|service| {
        service
            .borrow_mut()
            .insert(lesson.id, updated_lesson.clone());
    });

    Ok(updated_lesson)
}

#[ic_cdk::update]
fn delete_lesson(id: u64) -> Result<(), Error> {
    let _lesson = _get_lesson(&id).ok_or_else(|| Error::NotFound {
        msg: format!("lesson with id={} not found", id),
    })?;

    let course = _get_course(&_lesson.course_id).ok_or_else(|| Error::NotFound {
        msg: format!("course with id={} not found", _lesson.course_id),
    })?;

    is_course_creator(&course.clone())?;

    let updated_course_lesson: Vec<u64> = course.lessons.into_iter().filter(|&lesson_id| lesson_id != id).collect();
   
   let updated_course = Course {
    id: course.id,
    creator_principal: course.creator_principal,
    title: course.title,
    description: course.description,
    lessons: updated_course_lesson,
    created_at: course.created_at,
    updated_at: Some(time()),
   };


    // Delete the lesson
    LESSON_STORAGE.with(|service| service.borrow_mut().remove(&id));

    // Replace the existing course with the updated version
    COURSE_STORAGE.with(|service| {
        service
            .borrow_mut()
            .insert(course.id, updated_course.clone());
    });

    // Optionally, you may want to update associated courses or user progress

    Ok(())
}

#[ic_cdk::query]
fn get_certificate(id: u64) -> Result<Certificate, Error> {
    match _get_certificate(&id) {
        Some(certificate) => Ok(certificate),
        None => Err(Error::NotFound {
            msg: format!("a certificate with id={} not found", id),
        }),
    }
}

#[ic_cdk::update]
fn issue_certificate(user_id: u64, course_id: u64) -> Result<Certificate, Error> {
    // Check if the user completed the course
    let _course = _get_course(&course_id).ok_or_else(|| Error::NotFound {
        msg: format!("course with id={} not found", course_id),
    })?;
    // Add more conditions for certification criteria...
    is_course_creator(&_course)?;
    let certificate = Certificate {
        id: ID_COUNTER
            .with(|counter| {
                let current_value = *counter.borrow().get();
                counter.borrow_mut().set(current_value + 1)
            })
            .expect("cannot increment id counter"),
        course_id,
        user_id,
        issue_date: time(),
    };

    CERTIFICATE_STORAGE
        .with(|service| service.borrow_mut().insert(certificate.id, certificate.clone()));

    Ok(certificate)
}

#[ic_cdk::query]
fn verify_certificate(user_id: u64, certificate_id: u64) -> Result<bool, Error> {
    match CERTIFICATE_STORAGE.with(|service| service.borrow().get(&certificate_id)) {
        Some(certificate) => Ok(certificate.user_id == user_id),
        None => Err(Error::NotFound {
            msg: format!("certificate with id={} not found", certificate_id),
        }),
    }
}

#[ic_cdk::query]
fn get_user(id: u64) -> Result<User, Error> {
    match _get_user(&id) {
        Some(user) => Ok(user),
        None => Err(Error::NotFound {
            msg: format!("a user with id={} not found", id),
        }),
    }
}

#[ic_cdk::update]
fn register_user(username: String, public_key: String) -> Result<User, Error> {
    let id = ID_COUNTER
        .with(|counter| {
            let current_value = *counter.borrow().get();
            counter.borrow_mut().set(current_value + 1)
        })
        .expect("cannot increment id counter");
    let user = User {
        id,
        username,
        public_key,
    };
    USER_STORAGE.with(|service| service.borrow_mut().insert(user.id, user.clone()));
    Ok(user)
}

#[ic_cdk::update]
fn delete_user(id: u64) -> Result<(), Error> {
    let _user = _get_user(&id).ok_or_else(|| Error::NotFound {
        msg: format!("user with id={} not found", id),
    })?;

    // Delete the user
    USER_STORAGE.with(|service| service.borrow_mut().remove(&id));

    // Optionally, you may want to update associated certificates or user enrollments

    Ok(())
}

// Update the _get_message method to handle courses and lessons
fn _get_course(id: &u64) -> Option<Course> {
    COURSE_STORAGE.with(|service| service.borrow().get(id))
}

fn _get_lesson(id: &u64) -> Option<Lesson> {
    LESSON_STORAGE.with(|service| service.borrow().get(id))
}

fn _get_certificate(id: &u64) -> Option<Certificate> {
    CERTIFICATE_STORAGE.with(|service| service.borrow().get(id))
}

fn _get_user(id: &u64) -> Option<User> {
    USER_STORAGE.with(|service| service.borrow().get(id))
}

#[derive(candid::CandidType, Deserialize, Serialize)]
enum Error {
    NotFound { msg: String },
    NotCreator,
    InputValidationFailed {msg: String}
}

// need this to generate candid
ic_cdk::export_candid!();
