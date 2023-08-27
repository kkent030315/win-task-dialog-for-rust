#[cfg(windows)]
extern crate winapi;

#[cfg(windows)]
use widestring::U16CString;
#[cfg(windows)]
use winapi::shared::basetsd::LONG_PTR;
#[cfg(windows)]
use winapi::shared::minwindef::*;
#[cfg(windows)]
pub use winapi::shared::windef::HWND;
#[cfg(windows)]
use winapi::shared::winerror::S_OK;
#[cfg(windows)]
use winapi::um::commctrl::{
    TASKDIALOGCONFIG_u1, TASKDIALOGCONFIG_u2, TaskDialogIndirect, HRESULT, TASKDIALOGCONFIG,
    TASKDIALOG_BUTTON, TASKDIALOG_COMMON_BUTTON_FLAGS, TASKDIALOG_FLAGS,
};
#[cfg(windows)]
use winapi::um::commctrl::{
    TDE_CONTENT, TDE_EXPANDED_INFORMATION, TDE_FOOTER, TDE_MAIN_INSTRUCTION,
    TDM_UPDATE_ELEMENT_TEXT,
};
#[cfg(windows)]
pub use winapi::um::commctrl::{
    TDF_SHOW_MARQUEE_PROGRESS_BAR, TDF_SHOW_PROGRESS_BAR, TDM_SET_PROGRESS_BAR_MARQUEE,
    TDM_SET_PROGRESS_BAR_POS, TDN_BUTTON_CLICKED, TDN_CREATED, TDN_DESTROYED,
    TDN_HYPERLINK_CLICKED, TDN_NAVIGATED,
};
use winapi::um::commctrl::{TDM_NAVIGATE_PAGE, TDM_SET_BUTTON_ELEVATION_REQUIRED_STATE};
#[cfg(windows)]
use winapi::um::libloaderapi::GetModuleHandleA;
#[cfg(windows)]
use winapi::um::winnt::LPWSTR;
#[cfg(windows)]
use winapi::um::winuser::SendMessageA;

#[cfg(not(windows))]
type HWND = *mut usize;

#[cfg(not(windows))]
type HMODULE = *mut usize;

#[cfg(not(windows))]
type LPWSTR = *mut u16;

#[cfg(not(windows))]
#[allow(non_camel_case_types)]
type TASKDIALOG_FLAGS = u32;

#[cfg(not(windows))]
#[allow(non_camel_case_types)]
type TASKDIALOG_COMMON_BUTTON_FLAGS = u32;

use std::ptr::null_mut;
use std::{io::Error, usize};

pub type TaskDialogHyperlinkCallback = Option<fn(context: &str) -> ()>;

pub type TaskDialogWndProcCallback = Option<
    unsafe extern "system" fn(
        hwnd: HWND,
        msg: UINT,
        w_param: WPARAM,
        l_param: LPARAM,
        ref_data: *mut TaskDialogConfig,
    ) -> HRESULT,
>;

mod constants;
pub use constants::*;
pub struct TaskDialogConfig {
    pub parent: HWND,
    pub instance: HMODULE,
    pub flags: TASKDIALOG_FLAGS,
    pub common_buttons: TASKDIALOG_COMMON_BUTTON_FLAGS,
    pub window_title: Option<String>,
    pub main_instruction: Option<String>,
    pub content: Option<String>,
    pub verification_text: Option<String>,
    pub expanded_information: Option<String>,
    pub expanded_control_text: Option<String>,
    pub collapsed_control_text: Option<String>,
    pub footer: Option<String>,
    pub buttons: Vec<TaskDialogButton>,
    pub default_button: i32,
    pub radio_buttons: Vec<TaskDialogButton>,
    pub default_radio_buttons: i32,
    pub main_icon: Option<LPWSTR>,
    pub footer_icon: Option<LPWSTR>,
    /** When created dialog, the value set to HWND. */
    pub dialog_hwnd: HWND,
    /** When close the dialog, the value set to true, default is false. */
    pub is_destroyed: bool,
    pub hyperlink_callback: TaskDialogHyperlinkCallback,
    pub callback: TaskDialogWndProcCallback,
    pub cx_width: u32,
}

pub struct TaskDialogResource {
    pub parent: HWND,
    pub instance: HMODULE,
    pub flags: TASKDIALOG_FLAGS,
    pub common_buttons: TASKDIALOG_COMMON_BUTTON_FLAGS,
    pub window_title: Option<U16CString>,
    pub main_instruction: Option<U16CString>,
    pub content: Option<U16CString>,
    pub verification_text: Option<U16CString>,
    pub expanded_information: Option<U16CString>,
    pub expanded_control_text: Option<U16CString>,
    pub collapsed_control_text: Option<U16CString>,
    pub footer: Option<U16CString>,
    pub buttons_resource: Vec<U16CString>,
    pub buttons: Vec<TASKDIALOG_BUTTON>,
    pub default_button: i32,
    pub radio_buttons_resource: Vec<U16CString>,
    pub radio_buttons: Vec<TASKDIALOG_BUTTON>,
    pub default_radio_buttons: i32,
    pub main_icon: Option<LPWSTR>,
    pub footer_icon: Option<LPWSTR>,
    /** When created dialog, the value set to HWND. */
    pub dialog_hwnd: HWND,
    /** When close the dialog, the value set to true, default is false. */
    pub is_destroyed: bool,
    pub hyperlink_callback: TaskDialogHyperlinkCallback,
    pub callback: TaskDialogWndProcCallback,
    pub cx_width: u32,
}

impl Default for TaskDialogConfig {
    fn default() -> Self {
        TaskDialogConfig {
            parent: null_mut(),
            instance: null_mut(),
            flags: 0,
            common_buttons: TDCBF_CANCEL_BUTTON,
            window_title: None,
            main_instruction: None,
            content: None,
            verification_text: None,
            expanded_information: None,
            expanded_control_text: None,
            collapsed_control_text: None,
            footer: None,
            buttons: vec![],
            default_button: 0,
            radio_buttons: vec![],
            default_radio_buttons: 0,
            main_icon: None,
            footer_icon: None,
            dialog_hwnd: null_mut(),
            is_destroyed: false,
            hyperlink_callback: None,
            callback: None,
            cx_width: 0,
        }
    }
}

#[cfg(windows)]
impl TaskDialogConfig {
    /**
    Add TDF_SHOW_PROGRESS_BAR flag on marquee is false;

    Add TDF_SHOW_MARQUEE_PROGRESS_BAR flag on marquee is true;

    https://docs.microsoft.com/en-us/windows/win32/controls/progress-bar-control
    */
    pub fn enable_process_bar(&mut self, marquee: bool) {
        if marquee {
            if self.flags & TDF_SHOW_MARQUEE_PROGRESS_BAR != TDF_SHOW_MARQUEE_PROGRESS_BAR {
                self.flags = self.flags | TDF_SHOW_MARQUEE_PROGRESS_BAR;
            }
        } else {
            if self.flags & TDF_SHOW_PROGRESS_BAR != TDF_SHOW_PROGRESS_BAR {
                self.flags = self.flags | TDF_SHOW_PROGRESS_BAR;
            }
        }
    }

    /** Set status or animation time of marquee progress bar */
    pub fn set_process_bar_marquee(&mut self, enable: bool, time: isize) {
        if self.dialog_hwnd.is_null() {
            return;
        }
        unsafe {
            SendMessageA(
                self.dialog_hwnd,
                TDM_SET_PROGRESS_BAR_MARQUEE,
                if enable {
                    TRUE as usize
                } else {
                    FALSE as usize
                },
                time,
            );
        }
    }

    /** Set the percentage of the progress bar */
    pub fn set_process_bar(&mut self, percentage: usize) {
        if self.dialog_hwnd.is_null() {
            return;
        }
        unsafe {
            SendMessageA(self.dialog_hwnd, TDM_SET_PROGRESS_BAR_POS, percentage, 0);
        }
    }

    /** Set the content text */
    pub fn set_content(&mut self, content: &str) {
        if self.dialog_hwnd.is_null() {
            return;
        }
        let content_wchar = U16CString::from_str(content).unwrap();
        unsafe {
            SendMessageA(
                self.dialog_hwnd,
                TDM_UPDATE_ELEMENT_TEXT,
                TDE_CONTENT as _,
                content_wchar.as_ptr() as _,
            );
        }
    }

    /** Set the main instruction text */
    pub fn set_main_instruction(&mut self, main_instruction: &str) {
        if self.dialog_hwnd.is_null() {
            return;
        }
        let main_instruction_wchar = U16CString::from_str(main_instruction).unwrap();
        unsafe {
            SendMessageA(
                self.dialog_hwnd,
                TDM_UPDATE_ELEMENT_TEXT,
                TDE_MAIN_INSTRUCTION as _,
                main_instruction_wchar.as_ptr() as _,
            );
        }
    }

    /** Set the footer text */
    pub fn set_footer(&mut self, footer: &str) {
        if self.dialog_hwnd.is_null() {
            return;
        }
        let footer_wchar = U16CString::from_str(footer).unwrap();
        unsafe {
            SendMessageA(
                self.dialog_hwnd,
                TDM_UPDATE_ELEMENT_TEXT,
                TDE_FOOTER as _,
                footer_wchar.as_ptr() as _,
            );
        }
    }

    /** Set the expanded information text */
    pub fn set_expanded_information(&mut self, expanded_information: &str) {
        if self.dialog_hwnd.is_null() {
            return;
        }
        let expanded_information_wchar = U16CString::from_str(expanded_information).unwrap();
        unsafe {
            SendMessageA(
                self.dialog_hwnd,
                TDM_UPDATE_ELEMENT_TEXT,
                TDE_EXPANDED_INFORMATION as _,
                expanded_information_wchar.as_ptr() as _,
            );
        }
    }

    /** Set the button elevation state */
    pub fn set_button_elevation_required_state(&mut self, button_id: usize, enable: bool) {
        if self.dialog_hwnd.is_null() {
            return;
        }
        unsafe {
            SendMessageA(
                self.dialog_hwnd,
                TDM_SET_BUTTON_ELEVATION_REQUIRED_STATE,
                button_id,
                if enable { 1 } else { 0 },
            );
        }
    }

    #[inline(always)]
    pub fn navigate_page(&mut self, hwnd: HWND, config: &mut TaskDialogConfig) {
        unsafe {
            SendMessageA(hwnd, TDM_NAVIGATE_PAGE, 0, config as *mut _ as _);
        }
    }
}

#[cfg(not(windows))]
impl TaskDialogConfig {
    pub fn enable_process_bar(&mut self, _marquee: bool) {}
    pub fn set_process_bar_marquee(&mut self, _enable: bool, _time: isize) {}
    pub fn set_process_bar(&mut self, _percentage: usize) {}
    pub fn set_content(&mut self, content: &str) {}
    pub fn set_main_instruction(&mut self, main_instruction: &str) {}
    pub fn set_footer(&mut self, footer: &str) {}
    pub fn set_expanded_information(&mut self, expanded_information: &str) {}
    pub fn set_button_elevation_required_state(&mut self, button_id: usize, enable: bool) {}
}

pub struct TaskDialogButton {
    pub id: i32,
    pub text: String,
}

pub struct TaskDialogResult {
    pub button_id: i32,
    pub radio_button_id: i32,
    pub checked: bool,
}

impl Default for TaskDialogResult {
    fn default() -> Self {
        TaskDialogResult {
            button_id: 0,
            radio_button_id: 0,
            checked: false,
        }
    }
}

impl Into<TaskDialogResource> for &mut TaskDialogConfig {
    fn into(self) -> TaskDialogResource {
        unsafe {
            let mut res = TaskDialogResource {
                parent: self.parent,
                instance: self.instance,
                flags: self.flags,
                common_buttons: self.common_buttons,
                window_title: self
                    .window_title
                    .to_owned()
                    .map(|s| U16CString::from_os_str_unchecked(s)),
                main_instruction: self
                    .main_instruction
                    .to_owned()
                    .map(|s| U16CString::from_os_str_unchecked(s)),
                content: self
                    .content
                    .to_owned()
                    .map(|s| U16CString::from_os_str_unchecked(s)),
                verification_text: self
                    .verification_text
                    .to_owned()
                    .map(|s| U16CString::from_os_str_unchecked(s)),
                expanded_information: self
                    .expanded_information
                    .to_owned()
                    .map(|s| U16CString::from_os_str_unchecked(s)),
                expanded_control_text: self
                    .expanded_control_text
                    .to_owned()
                    .map(|s| U16CString::from_os_str_unchecked(s)),
                collapsed_control_text: self
                    .collapsed_control_text
                    .to_owned()
                    .map(|s| U16CString::from_os_str_unchecked(s)),
                footer: self
                    .footer
                    .to_owned()
                    .map(|s| U16CString::from_os_str_unchecked(s)),
                buttons_resource: self
                    .buttons
                    .iter()
                    .map(|b| U16CString::from_str_unchecked(&b.text))
                    .collect(),
                buttons: Vec::new(), // setup later
                default_button: self.default_button,
                radio_buttons_resource: self
                    .radio_buttons
                    .iter()
                    .map(|b| U16CString::from_str_unchecked(&b.text))
                    .collect(),
                radio_buttons: Vec::new(), // setup later
                default_radio_buttons: self.default_radio_buttons,
                main_icon: self.main_icon,
                footer_icon: self.footer_icon,
                dialog_hwnd: self.dialog_hwnd,
                is_destroyed: self.is_destroyed,
                hyperlink_callback: self.hyperlink_callback,
                callback: self.callback,
                cx_width: self.cx_width,
            };
            res.buttons = self
                .buttons
                .iter()
                .enumerate()
                .map(|(i, b)| TASKDIALOG_BUTTON {
                    nButtonID: b.id,
                    pszButtonText: res.buttons_resource[i].as_ptr(),
                })
                .collect();
            res.radio_buttons = self
                .radio_buttons
                .iter()
                .enumerate()
                .map(|(i, btn)| TASKDIALOG_BUTTON {
                    nButtonID: btn.id,
                    pszButtonText: res.radio_buttons_resource[i].as_ptr(),
                })
                .collect();
            res
        }
    }
}

impl From<&TaskDialogConfig> for TaskDialogResource {
    fn from(conf: &TaskDialogConfig) -> Self {
        unsafe { TaskDialogResource { ..conf.into() } }
    }
}

/** Show task dialog */
#[cfg(windows)]
pub fn show_task_dialog(conf: &mut TaskDialogConfig) -> Result<TaskDialogResult, Error> {
    let mut result = TaskDialogResult::default();
    let conf_ptr: *mut TaskDialogConfig = conf;
    let conf_long_ptr = conf_ptr as isize;

    let ret = unsafe {
        // Call GetModuleHandleA on conf.instance is null
        let instance = if conf.instance.is_null() {
            GetModuleHandleA(std::ptr::null())
        } else {
            conf.instance
        };

        let resource: TaskDialogResource = conf.into();

        // ICON
        let mut u1: TASKDIALOGCONFIG_u1 = Default::default();
        let mut u2: TASKDIALOGCONFIG_u2 = Default::default();
        if let Some(icon) = resource.main_icon {
            *u1.pszMainIcon_mut() = icon;
        }
        if let Some(icon) = resource.footer_icon {
            *u2.pszFooterIcon_mut() = icon;
        }

        unsafe extern "system" fn callback(
            hwnd: HWND,
            msg: UINT,
            _w_param: WPARAM,
            _l_param: LPARAM,
            lp_ref_data: LONG_PTR,
        ) -> HRESULT {
            let conf = std::mem::transmute::<isize, *mut TaskDialogConfig>(lp_ref_data);
            if msg == TDN_CREATED {
                (*conf).dialog_hwnd = hwnd;
            } else if msg == TDN_DESTROYED {
                (*conf).is_destroyed = true;
            } else if msg == TDN_HYPERLINK_CLICKED {
                let link = U16CString::from_ptr_str(_l_param as *const u16)
                    .to_string()
                    .unwrap();
                if let Some(callback) = (*conf).hyperlink_callback {
                    callback(&link);
                }
            }
            if let Some(callback) = (*conf).callback {
                return callback(hwnd, msg, _w_param, _l_param, lp_ref_data as _);
            }
            S_OK
        }

        let title: *const u16 = resource.window_title.map_or(null_mut(), |u| u.as_ptr());

        let config = TASKDIALOGCONFIG {
            cbSize: std::mem::size_of::<TASKDIALOGCONFIG>() as UINT,
            hwndParent: conf.parent,
            hInstance: instance,
            dwFlags: conf.flags,
            dwCommonButtons: conf.common_buttons,
            pszWindowTitle: title as _,
            pszMainInstruction: resource.main_instruction.map_or(null_mut(), |u| u.as_ptr()),
            pszContent: resource.content.map_or(null_mut(), |u| u.as_ptr()),
            pszVerificationText: resource
                .verification_text
                .map_or(null_mut(), |u| u.as_ptr()),
            pszExpandedInformation: resource
                .expanded_information
                .map_or(null_mut(), |u| u.as_ptr()),
            pszExpandedControlText: resource
                .expanded_control_text
                .map_or(null_mut(), |u| u.as_ptr()),
            pszCollapsedControlText: resource
                .collapsed_control_text
                .map_or(null_mut(), |u| u.as_ptr()),
            pszFooter: resource.footer.map_or(null_mut(), |u| u.as_ptr()),
            cButtons: resource.buttons.len() as UINT,
            pButtons: resource.buttons.as_slice().as_ptr(),
            nDefaultButton: conf.default_button,
            cRadioButtons: resource.radio_buttons.len() as UINT,
            pRadioButtons: resource.radio_buttons.as_slice().as_ptr(),
            nDefaultRadioButton: conf.default_radio_buttons,
            u1,
            u2,
            pfCallback: Some(callback),
            lpCallbackData: conf_long_ptr,
            cxWidth: conf.cx_width,
        };

        // Result
        let mut verify: BOOL = FALSE;
        let dialog_result = TaskDialogIndirect(
            &config,
            &mut result.button_id,
            &mut result.radio_button_id,
            &mut verify,
        );
        result.checked = verify != 0;
        dialog_result
    };
    if ret != 0 {
        Err(Error::last_os_error())
    } else {
        Ok(result)
    }
}

/** Show message dialog, the dialog have only the OK button */
#[cfg(windows)]
pub fn show_msg_dialog(
    title: Option<&str>,
    main_instruction: Option<&str>,
    content: Option<&str>,
    icon: Option<LPWSTR>,
) -> Option<Error> {
    let mut conf = TaskDialogConfig {
        common_buttons: TDCBF_OK_BUTTON,
        window_title: title.map(|s| s.to_string()),
        main_instruction: main_instruction.map(|s| s.to_string()),
        content: content.map(|s| s.to_string()),
        main_icon: icon,
        ..Default::default()
    };
    show_task_dialog(&mut conf).err()
}

#[cfg(not(windows))]
pub fn show_task_dialog(_conf: &TaskDialogConfig) -> Result<TaskDialogResult, Error> {
    Ok(TaskDialogResult::default())
}

#[cfg(not(windows))]
pub fn show_msg_dialog(
    _title: &str,
    _main_instruction: &str,
    _content: &str,
    _icon: *mut u16,
) -> Option<Error> {
    None
}
