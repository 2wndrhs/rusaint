use crate::webdynpro::element::ElementWrapper;

use self::{
    header_cell::SapTableHeaderCell, hierarchical_cell::SapTableHierarchicalCell,
    matrix_cell::SapTableMatrixCell, normal_cell::SapTableNormalCell,
    selection_cell::SapTableSelectionCell,
};

#[derive(Debug)]
pub enum SapTableCellWrapper<'a> {
    Normal(SapTableNormalCell<'a>),
    Header(SapTableHeaderCell<'a>),
    Hierarchical(SapTableHierarchicalCell<'a>),
    Matrix(SapTableMatrixCell<'a>),
    Selection(SapTableSelectionCell<'a>),
}

pub trait SapTableCell<'a> {
    fn content(&self) -> Option<&ElementWrapper<'a>>;
}

pub mod header_cell;
pub mod hierarchical_cell;
pub mod matrix_cell;
pub mod normal_cell;
pub mod selection_cell;
