extern crate win_task_dialog;

use std::time::Duration;
use std::{ptr, thread};
use win_task_dialog::*;
use winapi::um::combaseapi::CoInitializeEx;
use winapi::um::objbase::{COINIT_APARTMENTTHREADED, COINIT_DISABLE_OLE1DDE};

fn hyperlink_callback(context: &str) {
    println!("hyperlink_callback: {}", context);
}

unsafe extern "system" fn callback(
    hwnd: HWND,
    msg: u32,
    w_param: usize,
    l_param: isize,
    ref_data: *mut TaskDialogConfig,
) -> i32 {
    println!(
        "callback: hwnd={:?} msg={} wparam={:#X} lparam={:#X} ref_data={:?}",
        hwnd, msg, w_param, l_param, ref_data
    );
    0
}

fn main() {
    page_navigation();
    return;

    let mut conf = TaskDialogConfig {
        flags: TDF_USE_COMMAND_LINKS | TDF_ENABLE_HYPERLINKS,
        common_buttons: TDCBF_OK_BUTTON | TDCBF_CANCEL_BUTTON,
        window_title: Some("Title 标题".to_string()),
        main_instruction: Some("Привет".to_string()),
        content: Some("こんにちは".to_string()),
        verification_text: Some("VerificationText".to_string()),
        footer: Some("footer\n<a href=\"http://example.com\">example.com</a>".to_string()),
        buttons: vec![
            TaskDialogButton {
                id: 10,
                text: "One".to_string(),
            },
            TaskDialogButton {
                id: 11,
                text: "贰".to_string(),
            },
        ],
        default_button: 11,
        radio_buttons: vec![
            TaskDialogButton {
                id: 10,
                text: "Option 1".to_string(),
            },
            TaskDialogButton {
                id: 11,
                text: "Option 2".to_string(),
            },
        ],
        main_icon: Some(TD_SHIELD_ICON),
        footer_icon: Some(TD_INFORMATION_ICON),
        hyperlink_callback: Some(hyperlink_callback),
        callback: Some(callback),
        ..TaskDialogConfig::default()
    };

    let result = show_task_dialog(&mut conf).unwrap();
    println!(
        "{} {} {}",
        result.button_id, result.radio_button_id, result.checked,
    );

    show_dynamic_text_dialog();
    show_process_bar_marquee();
    show_process_bar();

    show_msg_dialog(
        Some("Title"),
        Some("Hi"),
        Some("Info"),
        Some(TD_INFORMATION_ICON),
    );
    show_msg_dialog(
        Some("Title"),
        Some("!!!"),
        Some("Error"),
        Some(TD_ERROR_ICON),
    );
}

// Show dynamic text dialog
fn show_dynamic_text_dialog() {
    let mut conf = TaskDialogConfig {
        window_title: Some("Dynamic Text".to_string()),
        main_instruction: Some("Main".to_string()),
        content: Some("Content".to_string()),
        footer: Some("Footer".to_string()),
        expanded_information: Some("Info".to_string()),
        ..Default::default()
    };

    let conf_ptr: *mut TaskDialogConfig = &mut conf;
    let conf_long_ptr = conf_ptr as isize;
    thread::spawn(move || unsafe {
        let conf = std::mem::transmute::<isize, *mut TaskDialogConfig>(conf_long_ptr);
        for i in 1..4 {
            thread::sleep(Duration::from_secs(1));
            (*conf).set_main_instruction(format!("Main {}", i).as_str());
            (*conf).set_content(format!("Content {}", i).as_str());
            (*conf).set_footer(format!("Footer {}", i).as_str());
            (*conf).set_expanded_information(format!("Info {}", i).as_str());
        }
        (*conf).set_main_instruction("Main !!!");
        (*conf).set_content("Content !!!");
        (*conf).set_footer("Footer !!!");
        (*conf).set_expanded_information("Info !!!");
    });

    show_task_dialog(&mut conf).unwrap();
}

// Process Bar Marquee
fn show_process_bar_marquee() {
    let mut conf = TaskDialogConfig {
        window_title: Some("Process Bar".to_string()),
        main_instruction: Some("Process Bar Marquee 1".to_string()),
        ..Default::default()
    };
    conf.enable_process_bar(true);

    let conf_ptr: *mut TaskDialogConfig = &mut conf;
    let conf_long_ptr = conf_ptr as isize;
    thread::spawn(move || unsafe {
        thread::sleep(Duration::from_secs(1));
        let conf = std::mem::transmute::<isize, *mut TaskDialogConfig>(conf_long_ptr);
        (*conf).set_process_bar_marquee(true, 100);
        thread::sleep(Duration::from_secs(5));
        (*conf).set_main_instruction("Process Bar Marquee 2");
        (*conf).set_process_bar_marquee(true, 1);
        thread::sleep(Duration::from_secs(3));
        (*conf).set_main_instruction("Process Bar Marquee Stop");
        (*conf).set_process_bar_marquee(false, 1);
    });

    show_task_dialog(&mut conf).unwrap();
}

// Process Bar (Not Marquee)
fn show_process_bar() {
    let mut conf = TaskDialogConfig {
        window_title: Some("Process Bar".to_string()),
        main_instruction: Some("Process Bar (Not Marquee)".to_string()),
        content: None,
        ..Default::default()
    };
    conf.enable_process_bar(false);

    let conf_ptr: *mut TaskDialogConfig = &mut conf;
    let conf_long_ptr = conf_ptr as isize;
    thread::spawn(move || unsafe {
        let conf = std::mem::transmute::<isize, *mut TaskDialogConfig>(conf_long_ptr);
        for i in 0..101 {
            if (*conf).is_destroyed {
                return;
            }
            thread::sleep(Duration::from_millis(20));
            (*conf).set_process_bar(i);
        }
    });

    show_task_dialog(&mut conf).unwrap();
}

fn page_navigation() {
    use winapi::um::processthreadsapi::GetCurrentThreadId;

    unsafe {
        CoInitializeEx(
            ptr::null_mut(),
            COINIT_APARTMENTTHREADED | COINIT_DISABLE_OLE1DDE,
        )
    };
    unsafe extern "system" fn page1_callback(
        _: HWND,
        msg: u32,
        w_param: usize,
        _: isize,
        ref_data: *mut TaskDialogConfig,
    ) -> i32 {
        println!("msg={} tid={}", msg, unsafe { GetCurrentThreadId() });
        if msg == TDN_NAVIGATED {
        } else if msg == TDN_BUTTON_CLICKED {
            // TDN_BUTTON_CLICKED
            if w_param as i32 == 1777 {
                let mut page2_conf = TaskDialogConfig {
                    window_title: Some("Page Navigation".to_owned()),
                    main_instruction: Some("Page #2".to_owned()),
                    common_buttons: TDCBF_CLOSE_BUTTON,
                    ..Default::default()
                };
                (*ref_data).navigate_page((*ref_data).dialog_hwnd, &mut page2_conf);
                return 1; // S_FALSE
            }
        }
        0
    }

    println!("tid={}", unsafe { GetCurrentThreadId() });

    let mut conf = TaskDialogConfig {
        window_title: Some("Page Navigation".to_owned()),
        main_instruction: Some("Page #1".to_owned()),
        callback: Some(page1_callback),
        common_buttons: TDCBF_CLOSE_BUTTON,
        buttons: vec![TaskDialogButton {
            id: 1777,
            text: "Continue".to_owned(),
        }],
        ..Default::default()
    };

    show_task_dialog(&mut conf).unwrap();
}
