use leptos::prelude::*;
use thaw::{
    Toast, ToastBody, ToastBodySubtitle, ToastFooter, ToastOptions, ToastPosition, ToastTitle,
};
use thaw::{ToastIntent, ToasterInjection};

use crate::utils::context::WebCallBackContext;

pub fn do_toast(ctx: &WebCallBackContext, title: &str, body: &str, intent: ToastIntent) {
    let toaster = ctx.toast_context;
    dispatch_toast_at_position(toaster, ToastPosition::TopEnd, intent, title, body, "", "");
}

pub fn dispatch_toast_at_position(
    toaster: ToasterInjection,
    position: ToastPosition,
    intent: ToastIntent,
    title: &str,
    body: &str,
    subtitle: &str,
    footer: &str,
) {
    let title = title.to_string();
    let body = body.to_string();
    let subtitle = subtitle.to_string();
    let footer = footer.to_string();

    toaster.dispatch_toast(
        move || {
            view! {
               <Toast>
                   <ToastTitle>{title}</ToastTitle>
                   <ToastBody>
                       <ToastBodySubtitle slot>
                           {subtitle}
                       </ToastBodySubtitle>
                       {body}
                   </ToastBody>
                   <ToastFooter>
                       {footer}
                       // <Link>Action</Link>
                       // <Link>Action</Link>
                   </ToastFooter>
               </Toast>
            }
        },
        ToastOptions::default()
            .with_position(position)
            .with_intent(intent),
    );
}
