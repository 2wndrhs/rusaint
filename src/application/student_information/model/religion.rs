use crate::{
    application::{student_information::StudentInformation, USaintClient},
    define_elements,
    webdynpro::{
        command::element::layout::TabStripTabSelectCommand,
        element::{
            action::Button, definition::ElementDefinition, layout::tab_strip::item::TabStripItem,
            selection::ComboBox, text::InputField,
        },
        error::WebDynproError,
    },
};

#[derive(Clone, Debug)]
/// 학생의 종교 정보
pub struct StudentReligionInformation {
    religion_type: Option<String>,
    start_date: Option<String>,
    church: Option<String>,
    church_man: Option<String>,
    baptism_level: Option<String>,
    baptism_grp: Option<String>,
    service_department: Option<String>,
    service_department_title: Option<String>,
    church_address: Option<String>,
    singeub: Option<String>,
    baptism_date: Option<String>,
    baptism_church: Option<String>,
    baptism_man: Option<String>,
    church_grp: Option<String>,
}

impl<'a> StudentReligionInformation {
    // 종교
    define_elements! {
        // 종교 탭
        TAB_RELIGION: TabStripItem<'a> = "ZCMW1001.ID_0001:VIW_MAIN.TAB_RELIGION";
        // 종교
        RELIGION_COD: ComboBox<'a> = "ZCMW1001.ID_0001:VIW_TAB_RELIGION.RELIGION_COD";
        // 신앙시작일
        BELIEF_START_DATE: InputField<'a> = "ZCMW1001.ID_0001:VIW_TAB_RELIGION.BELIEF_START_DAT";
        // 출석교회
        PRES_CHURCH: InputField<'a> = "ZCMW1001.ID_0001:VIW_TAB_RELIGION.PRES_CHURCH";
        // 담임목사
        CHURCH_MAN: InputField<'a> = "ZCMW1001.ID_0001:VIW_TAB_RELIGION.CHURCH_MAN";
        // 직분
        BAPTISM_LVL: ComboBox<'a> = "ZCMW1001.ID_0001:VIW_TAB_RELIGION.BAPTISM_LVL";
        // 교단
        BAPTISM_GRP: InputField<'a> = "ZCMW1001.ID_0001:VIW_TAB_RELIGION.BAPTISM_GRP";
        // 봉사부서
        SERVICE_DEPT_DES: InputField<'a> = "ZCMW1001.ID_0001:VIW_TAB_RELIGION.SERVICE_DEPT_DES";
        // 직책
        SERVICE_DEPT_LVL: InputField<'a> = "ZCMW1001.ID_0001:VIW_TAB_RELIGION.SERVICE_DEPT_LVL";
        // 교회주소
        CHURCH_ADDR: InputField<'a> = "ZCMW1001.ID_0001:VIW_TAB_RELIGION.CHURCH_ADDR";
        // 신급
        SINGEUB_DES: ComboBox<'a> = "ZCMW1001.ID_0001:VIW_TAB_RELIGION.SINGEUB_DES";
        // 세례일자
        BAPTISM_DT: InputField<'a> = "ZCMW1001.ID_0001:VIW_TAB_RELIGION.BAPTISM_DT";
        // 세례교회
        BAPTISM_CH: InputField<'a> = "ZCMW1001.ID_0001:VIW_TAB_RELIGION.BAPTISM_CH";
        // 집례목사
        BAPTISM_MAN: InputField<'a> = "ZCMW1001.ID_0001:VIW_TAB_RELIGION.BAPTISM_MAN";
        // 교단
        CHURCH_GRP: InputField<'a> = "ZCMW1001.ID_0001:VIW_TAB_RELIGION.CHURCH_GRP";
        #[allow(unused)]
        MODIFY_BUTTON: Button<'a> = "ZCMW1001.ID_0001:VIW_TAB_RELIGION.MODIFY_BUTTON";
        #[allow(unused)]
        SAVE_BUTTON: Button<'a> = "ZCMW1001.ID_0001:VIW_TAB_RELIGION.SAVE_BUTTON";
    }

    pub(crate) async fn with_client(client: &mut USaintClient) -> Result<Self, WebDynproError> {
        client
            .send(TabStripTabSelectCommand::new(
                StudentInformation::TAB_ADDITION,
                Self::TAB_RELIGION,
                2,
                0,
            ))
            .await?;
        Ok(Self {
            religion_type: Self::RELIGION_COD
                .from_body(client.body())?
                .value()
                .map(str::to_string),
            start_date: Self::BELIEF_START_DATE
                .from_body(client.body())?
                .value()
                .map(str::to_string),
            church: Self::PRES_CHURCH
                .from_body(client.body())?
                .value()
                .map(str::to_string),
            church_man: Self::CHURCH_MAN
                .from_body(client.body())?
                .value()
                .map(str::to_string),
            baptism_level: Self::BAPTISM_LVL
                .from_body(client.body())?
                .value()
                .map(str::to_string),
            baptism_grp: Self::BAPTISM_GRP
                .from_body(client.body())?
                .value()
                .map(str::to_string),
            service_department: Self::SERVICE_DEPT_DES
                .from_body(client.body())?
                .value()
                .map(str::to_string),
            service_department_title: Self::SERVICE_DEPT_LVL
                .from_body(client.body())?
                .value()
                .map(str::to_string),
            church_address: Self::CHURCH_ADDR
                .from_body(client.body())?
                .value()
                .map(str::to_string),
            singeub: Self::SINGEUB_DES
                .from_body(client.body())?
                .value()
                .map(str::to_string),
            baptism_date: Self::BAPTISM_DT
                .from_body(client.body())?
                .value()
                .map(str::to_string),
            baptism_church: Self::BAPTISM_CH
                .from_body(client.body())?
                .value()
                .map(str::to_string),
            baptism_man: Self::BAPTISM_MAN
                .from_body(client.body())?
                .value()
                .map(str::to_string),
            church_grp: Self::CHURCH_GRP
                .from_body(client.body())?
                .value()
                .map(str::to_string),
        })
    }
    
    /// 종교 분류를 반환합니다.
    pub fn religion_type(&self) -> Option<&str> {
        self.religion_type.as_ref().map(String::as_str)
    }
    
    /// 신앙시작일을 반환합니다.
    pub fn start_date(&self) -> Option<&str> {
        self.start_date.as_ref().map(String::as_str)
    }
    
    /// 출석교회를 반환합니다.
    pub fn church(&self) -> Option<&str> {
        self.church.as_ref().map(String::as_str)
    }
    
    /// 담임목사를 반환합니다.
    pub fn church_man(&self) -> Option<&str> {
        self.church_man.as_ref().map(String::as_str)
    }
    
    /// 직분을 반환합니다.
    pub fn baptism_level(&self) -> Option<&str> {
        self.baptism_level.as_ref().map(String::as_str)
    }
    
    /// 교단을 반환합니다.
    pub fn baptism_grp(&self) -> Option<&str> {
        self.baptism_grp.as_ref().map(String::as_str)
    }
    
    /// 봉사부서를 반환합니다.
    pub fn service_department(&self) -> Option<&str> {
        self.service_department.as_ref().map(String::as_str)
    }
    
    /// 직책을 반환합니다.
    pub fn service_department_title(&self) -> Option<&str> {
        self.service_department_title.as_ref().map(String::as_str)
    }
    
    /// 교회주소를 반환합니다.
    pub fn church_address(&self) -> Option<&str> {
        self.church_address.as_ref().map(String::as_str)
    }
    
    /// 신급을 반환합니다.
    pub fn singeub(&self) -> Option<&str> {
        self.singeub.as_ref().map(String::as_str)
    }
    
    /// 세례일자를 반환합니다.
    pub fn baptism_date(&self) -> Option<&str> {
        self.baptism_date.as_ref().map(String::as_str)
    }
    
    /// 세례교회를 반환합니다.
    pub fn baptism_church(&self) -> Option<&str> {
        self.baptism_church.as_ref().map(String::as_str)
    }
    
    /// 집례목사를 반환합니다.
    pub fn baptism_man(&self) -> Option<&str> {
        self.baptism_man.as_ref().map(String::as_str)
    }
    
    /// 교단을 반환합니다.
    pub fn church_grp(&self) -> Option<&str> {
        self.church_grp.as_ref().map(String::as_str)
    }
}
