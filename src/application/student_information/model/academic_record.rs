use std::collections::HashMap;

use serde::{de::{value::MapDeserializer, IntoDeserializer}, Deserialize};

use crate::{
    application::{student_information::StudentInformation, USaintClient}, define_elements, webdynpro::{command::element::layout::TabStripTabSelectCommand, element::{complex::{sap_table::FromSapTable, SapTable}, definition::ElementDefinition, layout::tab_strip::item::TabStripItem}, error::{ElementError, WebDynproError}}
};

#[derive(Clone, Debug)]
/// 학생의 학적상태 정보
pub struct StudentAcademicRecordInformation {
    records: Vec<StudentAcademicRecord>,
}

impl<'a> StudentAcademicRecordInformation {
    // 학적상태
    define_elements! {
        // 학적상태 탭
        TAB_READ_9600: TabStripItem<'a> = "ZCMW1001.ID_0001:VIW_MAIN.TAB_READ_9600";
        // 학적상태 표
        TABLE_9600: SapTable<'a> = "ZCMW1001.ID_0001:VIW_TAB_9600.TABLE";
    }

    pub(crate) async fn with_client(client: &mut USaintClient) -> Result<Self, WebDynproError> {
        client
            .send(TabStripTabSelectCommand::new(
                StudentInformation::TAB_ADDITION,
                Self::TAB_READ_9600,
                5,
                0,
            ))
            .await?;
        let table_element = Self::TABLE_9600.from_body(client.body())?;
        let table = table_element.table()?;
        let records = table.try_table_into::<StudentAcademicRecord>(client.body())?;
        Ok(Self { records })
    }
    
    /// 학생의 학적상태 기록들을 반환합니다.
    pub fn records(&self) -> &[StudentAcademicRecord] {
        &self.records
    }
}

#[derive(Clone, Debug, Deserialize)]
/// 학생의 학적상태 기록
pub struct StudentAcademicRecord {
    #[serde(rename(deserialize = "시작일"))]
    start_date: String,
    #[serde(rename(deserialize = "종료일"))]
    end_date: String,
    #[serde(rename(deserialize = "학년도"))]
    year: String,
    #[serde(rename(deserialize = "학기(내역)"))]
    term: String,
    #[serde(rename(deserialize = "내역"))]
    category: String,
    #[serde(rename(deserialize = "사유(내역)"))]
    reason: String,
    #[serde(rename(deserialize = "처리일자"))]
    process_date: String,
}

impl StudentAcademicRecord {
    /// 종료일을 반환합니다.
    pub fn end_date(&self) -> &str {
        &self.end_date
    }

    /// 학년도를 반환합니다.
    pub fn year(&self) -> &str {
        &self.year
    }

    /// 학기(내역)을 반환합니다.
    pub fn term(&self) -> &str {
        &self.term
    }

    /// 내역을 반환합니다.
    pub fn category(&self) -> &str {
        &self.category
    }

    /// 사유를 반환합니다.
    pub fn reason(&self) -> &str {
        &self.reason
    }

    /// 처리일자를 반환합니다.
    pub fn process_date(&self) -> &str {
        &self.process_date
    }
    
    /// 시작일을 반환합니다.
    pub fn start_date(&self) -> &str {
        &self.start_date
    }
}

impl<'a> FromSapTable<'a> for StudentAcademicRecord {
    fn from_table(
        body: &'a crate::webdynpro::client::body::Body,
        header: &'a crate::webdynpro::element::complex::sap_table::SapTableHeader<'a>,
        row: &'a crate::webdynpro::element::complex::sap_table::SapTableRow<'a>,
    ) -> Result<Self, crate::webdynpro::error::WebDynproError> {
        let map_string = row.try_row_into::<HashMap<String, String>>(header, body)?;
            let map_de: MapDeserializer<_, serde::de::value::Error> = map_string.into_deserializer();
            Ok(StudentAcademicRecord::deserialize(map_de).map_err(|e| {
                ElementError::InvalidContent {
                    element: row.table_def().id().to_string(),
                    content: e.to_string(),
                }
            })?)
    }
}

