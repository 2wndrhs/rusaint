use getset::Getters;
use std::{borrow::Cow, cell::OnceCell, ops::Deref};

use scraper::Selector;
use serde::Deserialize;

use crate::webdynpro::{
    element::{
        complex::sap_table::property::SapTableCellDesign, Element, ElementWrapper, SubElement,
        SubElementDef,
    },
    error::WebDynproError,
};

use super::{SapTableCell, SapTableCellWrapper};

/// 매트릭스 테이블의 셀
#[derive(custom_debug_derive::Debug)]
pub struct SapTableMatrixCell<'a> {
    id: Cow<'static, str>,
    #[debug(skip)]
    element_ref: scraper::ElementRef<'a>,
    lsdata: OnceCell<SapTableMatrixCellLSData>,
    content: OnceCell<Option<ElementWrapper<'a>>>,
}

#[derive(Getters, Deserialize, Debug, Default)]
#[allow(unused)]
#[get = "pub"]
pub struct SapTableMatrixCellLSData {
    #[serde(rename = "0")]
    cell_background_design: Option<SapTableCellDesign>,
    #[serde(rename = "1")]
    header_cell_ids: Option<String>,
    #[serde(rename = "2")]
    row_header_cell_ids: Option<String>,
    #[serde(rename = "3")]
    custom_data: Option<String>,
}

impl<'a> SapTableCell<'a> for SapTableMatrixCell<'a> {
    fn content(&self) -> Option<&ElementWrapper<'a>> {
        self.content
            .get_or_init(|| {
                let content_selector = Selector::parse(":root [ct]").unwrap();
                ElementWrapper::dyn_elem(
                    self.element_ref
                        .select(&content_selector)
                        .next()?
                        .to_owned(),
                )
                .ok()
            })
            .as_ref()
    }
}

impl<'a> SubElement<'a> for SapTableMatrixCell<'a> {
    const SUBCONTROL_ID: &'static str = "MC";
    const ELEMENT_NAME: &'static str = "SapTableMatrixCell";

    type SubElementLSData = SapTableMatrixCellLSData;

    fn lsdata(&self) -> &Self::SubElementLSData {
        self.lsdata.get_or_init(|| {
            let Ok(lsdata_obj) = Self::lsdata_elem(self.element_ref) else {
                    return Self::SubElementLSData::default();
                };
            serde_json::from_value::<Self::SubElementLSData>(lsdata_obj)
                .unwrap_or(Self::SubElementLSData::default())
        })
    }

    fn from_elem<Parent: Element<'a>>(
        elem_def: SubElementDef<'a, Parent, Self>,
        element: scraper::ElementRef<'a>,
    ) -> Result<Self, WebDynproError> {
        Ok(Self::new(elem_def.id.to_owned(), element))
    }

    fn id(&self) -> &str {
        &self.id
    }

    fn element_ref(&self) -> &scraper::ElementRef<'a> {
        &self.element_ref
    }
}

impl<'a> SapTableMatrixCell<'a> {
    pub const fn new(id: Cow<'static, str>, element_ref: scraper::ElementRef<'a>) -> Self {
        Self {
            id,
            element_ref,
            lsdata: OnceCell::new(),
            content: OnceCell::new(),
        }
    }

    /// 셀을 [`SapTableCellWrapper`]로 감쌉니다.
    pub fn wrap(self) -> SapTableCellWrapper<'a> {
        SapTableCellWrapper::Matrix(self)
    }
}

impl<'a> Deref for SapTableMatrixCell<'a> {
    type Target = SapTableMatrixCellLSData;

    fn deref(&self) -> &Self::Target {
        self.lsdata()
    }
}