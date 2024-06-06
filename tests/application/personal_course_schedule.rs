use rusaint::{
    application::{personal_course_schedule::PersonalCourseSchedule, USaintClientBuilder},
    model::SemesterType,
    ApplicationError, RusaintError,
};
use serial_test::serial;

use crate::get_session;

#[tokio::test]
#[serial]
async fn schedule() {
    let session = get_session().await.unwrap();
    let mut app = USaintClientBuilder::new()
        .session(session)
        .build_into::<PersonalCourseSchedule>()
        .await
        .unwrap();
    let info = app.schedule(2022, SemesterType::Two).await.unwrap();
    println!("{:?}", info);
}

#[tokio::test]
#[serial]
async fn no_schedule() {
    let session = get_session().await.unwrap();
    let mut app = USaintClientBuilder::new()
        .session(session)
        .build_into::<PersonalCourseSchedule>()
        .await
        .unwrap();
    let info = app.schedule(2024, SemesterType::Two).await.unwrap_err();
    assert!(matches!(
        info,
        RusaintError::ApplicationError(ApplicationError::NoScheduleInformation)
    ));
    println!("{:?}", info);
}